import { promises as fs } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoDir = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const sourceRoot = path.join(repoDir, 'docs');
const docsRoot = path.join(repoDir, 'src', 'content', 'docs');
const specRoot = path.join(docsRoot, 'spec');
const generatedRoot = path.join(repoDir, 'src', 'generated');
const homeFile = path.join(docsRoot, 'index.mdx');
const siteDataFile = path.join(generatedRoot, 'site-data.js');
const repoEditBase = 'https://github.com/joaogabrielfer/juz-vibes/edit/main/site';

await main();

async function main() {
	const versions = await listVersionDirectories(sourceRoot);

	if (versions.length === 0) {
		throw new Error('No documentation versions found in docs/.');
	}

	const currentVersion = versions[0];
	const currentDir = path.join(sourceRoot, currentVersion);
	const indexPath = path.join(currentDir, 'index.md');
	const indexSource = await readFile(indexPath);
	const siteTitle = extractSiteTitle(indexSource);
	const homeTitle = extractHeading(indexSource) || `${siteTitle} Specification`;
	const sections = parseIndex(indexSource);

	if (sections.length === 0) {
		throw new Error(`No navigation entries found in ${path.relative(repoDir, indexPath)}.`);
	}

	const allSources = await fs.readdir(currentDir);
	const markdownFiles = allSources.filter((file) => file.endsWith('.md') && file !== 'index.md');
	const availableSlugs = new Set(markdownFiles.map((file) => path.basename(file, '.md')));
	const navEntries = sections.flatMap((section) => section.entries);
	const navEntryBySlug = new Map(navEntries.map((entry) => [entry.slug, entry]));
	const titleBySlug = new Map();

	for (const entry of navEntries) {
		if (entry.label) {
			titleBySlug.set(entry.slug, entry.label);
		}
	}

	const docs = [];

	for (const fileName of markdownFiles) {
		const slug = path.basename(fileName, '.md');
		const sourcePath = path.join(currentDir, fileName);
		const sourceText = await readFile(sourcePath);
		const sourceTitle = extractHeading(sourceText);
		const title = titleBySlug.get(slug) ?? sourceTitle ?? humanizeSlug(slug);
		const body = prepareMarkdownBody(removeLeadingHeading(sourceText));
		const summary = extractDescription(body);
		const description =
			summary && summary.length >= 32 ? summary : `Reference notes for ${title}.`;
		const navEntry = navEntryBySlug.get(slug);

		titleBySlug.set(slug, title);
		docs.push({
			slug,
			title,
			description,
			order: navEntry?.order ?? 999,
			section: navEntry?.section ?? '',
			sourceFile: fileName,
			body,
		});
	}

	docs.sort((left, right) => left.order - right.order || left.title.localeCompare(right.title));

	const hydratedSections = sections
		.map((section) => {
			const entries = section.entries
				.filter((entry) => availableSlugs.has(entry.slug))
				.map((entry) => ({
					...entry,
					label: titleBySlug.get(entry.slug) ?? humanizeSlug(entry.slug),
					description: docs.find((doc) => doc.slug === entry.slug)?.description ?? '',
				}));

			const missingEntries = section.entries.filter((entry) => !availableSlugs.has(entry.slug));
			for (const entry of missingEntries) {
				console.warn(`Skipping missing doc "${entry.slug}" referenced in ${currentVersion}/index.md.`);
			}

			return {
				label: section.label,
				order: section.order,
				entries,
			};
		})
		.filter((section) => section.entries.length > 0);

	const usedSlugs = new Set(hydratedSections.flatMap((section) => section.entries.map((entry) => entry.slug)));
	const orphanDocs = docs
		.filter((doc) => !usedSlugs.has(doc.slug))
		.map((doc) => ({
			slug: doc.slug,
			label: doc.title,
			description: doc.description,
			order: doc.order,
		}));

	if (orphanDocs.length > 0) {
		hydratedSections.push({
			label: 'Additional Pages',
			order: hydratedSections.length + 1,
			entries: orphanDocs,
		});
	}

	await fs.mkdir(docsRoot, { recursive: true });
	await fs.mkdir(generatedRoot, { recursive: true });
	await fs.rm(specRoot, { recursive: true, force: true });
	await fs.mkdir(specRoot, { recursive: true });

	for (const doc of docs) {
		const sourceRelativePath = toPosix(path.relative(repoDir, path.join(currentDir, doc.sourceFile)));
		const generatedPath = path.join(specRoot, `${doc.slug}.md`);
		const frontmatter = [
			'---',
			`title: ${yamlString(doc.title)}`,
			`description: ${yamlString(doc.description)}`,
			`editUrl: ${yamlString(`${repoEditBase}/${sourceRelativePath}`)}`,
			'sidebar:',
			`  order: ${doc.order}`,
			`  label: ${yamlString(doc.title)}`,
			'---',
			'',
		].join('\n');
		const body = convertWikilinks(doc.body, titleBySlug);
		await fs.writeFile(generatedPath, `${frontmatter}${body.trim()}\n`);
	}

	const revision = extractRevision(indexSource);
	const homeDescription = revision
		? `${revision} documentation for ${siteTitle}.`
		: `Working draft documentation for ${siteTitle}.`;

	await fs.writeFile(
		homeFile,
		buildHomePage({
			homeTitle,
			currentVersion,
			revision,
			description: homeDescription,
			sections: hydratedSections,
			indexEditUrl: `${repoEditBase}/${toPosix(path.relative(repoDir, indexPath))}`,
		})
	);

	await fs.writeFile(
		siteDataFile,
		buildSiteDataFile({
			siteTitle,
			siteDescription: homeDescription,
			sections: hydratedSections,
		})
	);
}

async function listVersionDirectories(root) {
	const entries = await fs.readdir(root, { withFileTypes: true });

	return entries
		.filter((entry) => entry.isDirectory())
		.filter((entry) => parseVersionDirectory(entry.name) !== null)
		.map((entry) => entry.name)
		.sort(compareVersionsDesc);
}

function compareVersionsDesc(left, right) {
	const leftVersion = parseVersionDirectory(left);
	const rightVersion = parseVersionDirectory(right);
	const leftParts = [leftVersion.major, leftVersion.minor];
	const rightParts = [rightVersion.major, rightVersion.minor];

	for (let index = 0; index < leftParts.length; index += 1) {
		const leftValue = leftParts[index];
		const rightValue = rightParts[index];

		if (leftValue !== rightValue) {
			return rightValue - leftValue;
		}
	}

	return right.localeCompare(left);
}

function parseVersionDirectory(value) {
	const match = value.match(/^v?(\d+)\.(\d+)$/);

	if (!match) {
		return null;
	}

	return {
		major: Number.parseInt(match[1], 10),
		minor: Number.parseInt(match[2], 10),
	};
}

function parseIndex(source) {
	const lines = normalizeSource(source).split('\n');
	const sections = [];
	let currentSection = null;
	let sectionOrder = 0;
	let entryOrder = 0;

	for (const rawLine of lines) {
		const line = rawLine.trim();

		if (/^##\s+/.test(line)) {
			const label = line.replace(/^##\s+/, '').trim();

			if (label.toLowerCase() === 'sections') {
				currentSection = createSection('Specification', ++sectionOrder);
			} else {
				currentSection = createSection(label, ++sectionOrder);
			}

			sections.push(currentSection);
			continue;
		}

		const legacyMatch = line.match(/^- \[\[([^\]]+)\]\] - (.+)$/);
		if (legacyMatch) {
			if (!currentSection) {
				currentSection = createSection('Specification', ++sectionOrder);
				sections.push(currentSection);
			}

			entryOrder += 1;
			currentSection.entries.push({
				slug: legacyMatch[1],
				label: stripMarkdown(legacyMatch[2]),
				order: entryOrder,
				section: currentSection.label,
			});
			continue;
		}

		const bareMatch = line.match(/^- \[\[([^\]]+)\]\]$/);
		if (bareMatch) {
			if (!currentSection) {
				currentSection = createSection('Specification', ++sectionOrder);
				sections.push(currentSection);
			}

			entryOrder += 1;
			currentSection.entries.push({
				slug: bareMatch[1],
				label: '',
				order: entryOrder,
				section: currentSection.label,
			});
		}
	}

	return sections.filter((section) => section.entries.length > 0);
}

function createSection(label, order) {
	return {
		label,
		order,
		entries: [],
	};
}

function extractRevision(source) {
	const match = source.match(/^###\s+(.+)$/m);
	return match?.[1]?.trim() ?? '';
}

function extractHeading(source) {
	const normalized = normalizeSource(source);
	const match = normalized.match(/^#{1,6}\s+(.+)$/m);
	return match?.[1] ? stripMarkdown(match[1]).trim() : '';
}

function extractSiteTitle(source) {
	const heading = extractHeading(source);
	if (!heading) {
		return 'Element Language';
	}

	return heading.replace(/\s+Specification$/i, '').trim() || heading.trim();
}

function removeLeadingHeading(source) {
	const normalized = normalizeSource(source);
	const lines = normalized.split('\n');
	let index = 0;

	while (index < lines.length && lines[index].trim() === '') {
		index += 1;
	}

	if (index < lines.length && /^#{1,6}\s+/.test(lines[index])) {
		index += 1;
		while (index < lines.length && lines[index].trim() === '') {
			index += 1;
		}
	}

	return lines.slice(index).join('\n').trim();
}

function prepareMarkdownBody(source) {
	const normalized = normalizeSource(source);
	const lines = normalized.split('\n');
	let insideFence = false;

	return lines
		.map((line) => {
			const fenceMatch = line.match(/^(```+|~~~+)(\S+)?(.*)$/);

			if (fenceMatch) {
				insideFence = !insideFence;
				const [, fence, language = '', rest = ''] = fenceMatch;
				const normalizedLanguage = language.toLowerCase() === 'rs' ? 'rust' : language;

				return `${fence}${normalizedLanguage}${rest}`;
			}

			return insideFence ? line : normalizeTableRow(normalizeQuotedOperators(line));
		})
		.join('\n');
}

function normalizeQuotedOperators(line) {
	return line.replace(/'([<>|$&+#*=~!?:./-]{2,})'/g, '`$1`');
}

function normalizeTableRow(line) {
	const trimmed = line.trim();

	if (!trimmed.startsWith('|') || !trimmed.endsWith('|')) {
		return line;
	}

	let nextLine = '';
	let insideInlineCode = false;

	for (let index = 0; index < line.length; index += 1) {
		const char = line[index];
		const previous = line[index - 1];

		if (char === '`' && previous !== '\\') {
			insideInlineCode = !insideInlineCode;
			nextLine += char;
			continue;
		}

		if (insideInlineCode && char === '|') {
			nextLine += previous === '\\' ? char : '\\|';
			continue;
		}

		nextLine += char;
	}

	return nextLine.replace(/^\|\s*\|/, '| Comparison |');
}

function convertWikilinks(source, titleBySlug) {
	const normalized = normalizeSource(source);
	const lines = normalized.split('\n');
	let insideFence = false;

	return lines
		.map((line) => {
			if (/^```/.test(line.trim())) {
				insideFence = !insideFence;
				return line;
			}

			if (insideFence) {
				return line;
			}

			return line.replace(/\[\[([^\]]+)\]\]/g, (_match, slug) => {
				const label = titleBySlug.get(slug) ?? humanizeSlug(slug);
				return `[${label}](../${slug}/)`;
			});
		})
		.join('\n');
}

function extractDescription(source) {
	const normalized = normalizeSource(source);
	const lines = normalized.split('\n');
	let insideFence = false;
	let buffer = [];

	for (const rawLine of lines) {
		const line = rawLine.trim();

		if (/^```/.test(line)) {
			insideFence = !insideFence;
			buffer = [];
			continue;
		}

		if (insideFence || line === '' || /^#{1,6}\s+/.test(line) || /^[-*]\s+/.test(line)) {
			if (buffer.length > 0) {
				break;
			}
			continue;
		}

		buffer.push(line);
	}

	if (buffer.length === 0) {
		return '';
	}

	return truncate(stripMarkdown(buffer.join(' ')), 180);
}

function stripMarkdown(value) {
	return value
		.replace(/`([^`]+)`/g, '$1')
		.replace(/\[\[([^\]]+)\]\]/g, '$1')
		.replace(/\[([^\]]+)\]\([^)]+\)/g, '$1')
		.replace(/[*~]/g, '')
		.replace(/\s+/g, ' ');
}

function truncate(value, maxLength) {
	if (value.length <= maxLength) {
		return value.trim();
	}

	const shortened = value.slice(0, maxLength);
	const boundary = shortened.lastIndexOf(' ');

	return (boundary > 0 ? shortened.slice(0, boundary) : shortened).trim();
}

function buildHomePage({ homeTitle, currentVersion, revision, description, sections, indexEditUrl }) {
	const sectionBlocks = sections
		.map((section) => {
			const items = section.entries
				.map(
					(entry, index) =>
						`      <li>
        <a href="./spec/${entry.slug}/">
          <span class="spec-index-number">${String(index + 1).padStart(2, '0')}</span>
          <span class="spec-index-copy">
            <strong>${escapeHtml(entry.label)}</strong>
            <span>${escapeHtml(entry.description)}</span>
          </span>
        </a>
      </li>`
				)
				.join('\n');

			return `<section class="spec-section">
  <div class="spec-section-heading">
    <h2>${escapeHtml(section.label)}</h2>
    <p>${escapeHtml(describeSection(section.label))}</p>
  </div>
  <ol class="spec-index">
${items}
  </ol>
</section>`;
		})
		.join('\n\n');

	return `---
title: ${yamlString(homeTitle)}
description: ${yamlString(description)}
editUrl: ${yamlString(indexEditUrl)}
template: splash
tableOfContents: false
---

> ${revision || 'Working draft'} published from \`${currentVersion}\`.

The documentation is organized by reader task and system boundary instead of feature arrival order.

<div class="spec-sections">
${sectionBlocks}
</div>
`;
}

function describeSection(label) {
	const descriptions = {
		'Start Here': 'Orientation pages that explain the language direction, status, and examples before you dive into the reference.',
		'Language Reference': 'Parser-visible syntax, typing rules, evaluation rules, and compiler-owned semantics.',
		'Standard Library': 'User-level modules, traits, notations, and core library surfaces outside the compiler proper.',
		Toolchain: 'Build flow, plugins, runtime, and lifecycle details for compiling and running Element projects.',
		'Design Notes': 'Open questions, changelog notes, and authoring conventions for the evolving specification.',
		Specification: 'Reference pages listed in source order.',
		'Additional Pages': 'Generated pages that exist in the source tree but are not currently listed in the top-level index.',
	};

	return descriptions[label] ?? 'Documentation pages grouped from the source index.';
}

function buildSiteDataFile({ siteTitle, siteDescription, sections }) {
	const sidebar = [
		{ label: 'Home', slug: 'index' },
		...sections.map((section) => ({
			label: section.label,
			items: section.entries.map((entry) => ({
				label: entry.label,
				slug: `spec/${entry.slug}`,
			})),
		})),
	];

	return `export const siteTitle = ${JSON.stringify(siteTitle)};
export const siteDescription = ${JSON.stringify(siteDescription)};
export const docsSidebar = ${JSON.stringify(sidebar, null, 2)};
`;
}

function normalizeSource(source) {
	return source.replace(/^\uFEFF/, '').replace(/\r\n/g, '\n');
}

function humanizeSlug(slug) {
	return slug
		.replace(/^\d+-/, '')
		.split('-')
		.map((part) => (part.length > 0 ? part[0].toUpperCase() + part.slice(1) : part))
		.join(' ');
}

function yamlString(value) {
	return JSON.stringify(value ?? '');
}

function escapeHtml(value) {
	return String(value)
		.replace(/&/g, '&amp;')
		.replace(/</g, '&lt;')
		.replace(/>/g, '&gt;')
		.replace(/"/g, '&quot;')
		.replace(/{/g, '&#123;')
		.replace(/}/g, '&#125;');
}

function toPosix(value) {
	return value.split(path.sep).join('/');
}

async function readFile(filePath) {
	return normalizeSource(await fs.readFile(filePath, 'utf8'));
}
