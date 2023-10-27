import { defaultLang, showDefaultLang, ui } from "./ui";
import { getCollection } from "astro:content";

export function getLangFromUrl(url: URL) {
	const [, lang] = url.pathname.split("/");
	if (lang in ui) return lang as keyof typeof ui;
	return defaultLang;
}

export function useTranslations(lang: keyof typeof ui) {
	return function t(key: keyof typeof ui[typeof defaultLang]) {
		return ui[lang][key] || ui[defaultLang][key];
	};
}

export function useTranslatedPath(lang: keyof typeof ui) {
	return function translatePath(path: string, l: string = lang) {
		return !showDefaultLang && l === defaultLang ? path : `/${l}${path}`;
	};
}

//
export async function getLangCollection(collection: string) {
	const pages = await getCollection(collection);
	const langPagesMap = pages
		.map((page) => {
			const [lang, ...slug] = page.id
				.substring(0, page.id.lastIndexOf("."))
				?.split("/");
			page.slug = slug.join("/") || undefined;
			return { lang, page };
		})
		.reduce((lpMap, langPage) => {
			if (lpMap[langPage.lang]) {
				lpMap[langPage.lang].push(langPage.page);
			} else {
				lpMap[langPage.lang] = [langPage.page];
			}
			return lpMap;
		}, {});
	const langPagesMapSorted = {};
	for (const [key, value] of Object.entries(langPagesMap)) {
		langPagesMapSorted[key] = value.sort(
			(a, b) => a.data.sortOrder - b.data.sortOrder,
		);
	}
	return langPagesMapSorted;
}