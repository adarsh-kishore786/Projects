const article = document.querySelector('.article-content');

if (article) {
  const text = article.textContent;

  const wordMatchRegExp = /[^\s]+/g;
  const words = text.matchAll(wordMatchRegExp);

  const wordCount = [...words].length;
  const readingTime = Math.round(wordCount / 200);
  const badge = document.createElement("p");

  // badge.classList.add("color-secondary-text", "type--caption");
  badge.textContent = `⏱️ ${readingTime} min read`;
  const anchor = document.querySelector('.entry-title');

  anchor.insertAdjacentElement('afterend', badge);
}
