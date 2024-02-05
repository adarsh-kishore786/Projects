async function groupTabs(tabIds) {
  const group = await chrome.tabs.group({ tabIds });
  await chrome.tabGroups.update(group, { title: "Docs", collapsed: true, color: "cyan" });
}

async function groupUngroupTabs(tabs) {
  const tabIds = tabs.map(({ id }) => id);
  if (!tabIds.length) {
    return;
  }

  const groupIds = tabs.map(({ groupId }) => groupId);
  if (groupIds.length === 1 && groupIds[0] === -1) {
    await groupTabs(tabIds);
    return;
  }

  for (let i = 0; i < groupIds.length-1; i++) {
    console.log(groupIds[i]);
    console.log(groupIds[i+1]);
    if (groupIds[i] === -1 || groupIds[i] !== groupIds[i+1]) {
      await groupTabs(tabsIds);
      return;
    } 
  }

  console.log(tabs);
  await chrome.tabs.ungroup(tabIds);
  console.log(tabs);
}
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
button.addEventListener("click", () => groupUngroupTabs(tabs));
