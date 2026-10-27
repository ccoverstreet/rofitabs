let port = browser.runtime.connectNative("rofitabs");

async function postMessageToPort(port, msgId, data) {
	out = {
		"id": msgId,
		"data": data
	};

	port.postMessage(out);
}

browser.tabs.query({})
	.then((query) => {
		postMessageToPort(port, "tabs", query)		
	});

console.log("Rofitabs initializing");
console.log(browser.tabs);
console.log(port);

port.onMessage.addListener(async (msg) => {
	console.log("MESSAGE", msg);
	console.log(msg["id"], msg["id"] === "activate");

	if (msg["id"] === "activate") {
		const id = parseInt(msg["data"], 10);
		console.log(id);
		const tabDetails = await browser.tabs.get(id);
		browser.tabs.update(id, { active: true });
		browser.windows.update(tabDetails["windowId"], { focused: true });
	}
})


browser.tabs.onCreated.addListener(async (addedInfo) => {
	console.log(addedInfo);
	let query = await browser.tabs.query({});
	postMessageToPort(port, "tabs", query)
})

browser.tabs.onRemoved.addListener(async (tabId, removeInfo) => {
	console.log(removeInfo);
	let query = await browser.tabs.query({});
	postMessageToPort(port, "tabs", query)
})


