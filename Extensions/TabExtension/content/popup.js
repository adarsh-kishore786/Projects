const tabs = await chrome.tabs.query({
  url: [
    "https://developer.chrome.com/docs/webstore/*",
    "https://developer.chrome.com/docs/extensions/*",
    // "https://*/*",
    // "http://*/*",
  ]
})

const collator = new Intl.Collator();
tabs.sort((a, b) => collator.compare(a.title, b.title));

const template = document.getElementById("li_template");
const elements = new Set();
for (const tab of tabs) {
  const element = template.content.firstElementChild.cloneNode(true);

  const title = tab.title.split("-")[0].trim();
  const pathname = new URL(tab.url).pathname.slice("/docs".length);

  element.querySelector(".title").textContent = title;
  element.querySelector(".pathname").textContent = pathname;
  element.querySelector("a").addEventListener("click", async () => {
    await chrome.tabs.update(tab.id, { active: true });
    await chrome.windows.update(tab.windowId, { focused: true });
  });

  elements.add(element);
}
document.querySelector("ul").append(...elements);

const button = document.querySelector("button");
button.addEventListener("click", async() => {
  const tabIds = tabs.map(({ id }) => id);
  if (tabIds.length) {
    const groupIds = tabs.map(({ groupId }) => groupId);
    let flag = 1;
    for (let i = 0; i < groupIds.length-1; i++) {
      console.log(groupIds[i]);
      console.log(groupIds[i+1]);
      if (groupIds[i] === -1 || groupIds[i] !== groupIds[i+1]) {
        const group = await chrome.tabs.group({ tabIds });
        await chrome.tabGroups.update(group, { title: "Docs", collapsed: true, color: "cyan" });
        flag = 0;
        break;
      } 
    }
    if (flag === 1) {
      const ungroup = chrome.tabs.ungroup(tabIds);
      await chrome.tabGroups.update(ungroup);
    }
    console.log(flag);
  }
});
