const apiLocation = window.location.protocol + "//" + window.location.host + "/simpleshortener/api/";
var token = null;
var username = null;
var password = null;
var totalUrls = 0;
async function onLoad() {
  let user = window.localStorage.getItem("username");
  if (user === null) return;

  document.getElementById("username-input").value = user;

  await login();
}

async function logout() {
  window.localStorage.removeItem("username");
  window.localStorage.removeItem("password");
  await fetch(apiLocation + "token/invalidate/" + token, {
      method: 'POST'
    });

  window.location.reload();
}

async function login() {
  const statusElement = document.getElementById("login-status");
  const usernameField = document.getElementById("username-input");
  const passwordField = document.getElementById("password-input");
  statusElement.innerHTML = "Loading...";

  password = passwordField.value
  if (password === "") { password = window.localStorage.getItem("password"); }
  if (password === null) return;

  username = usernameField.value;

  console.log(
    "logging in with username " + username + " and password with length " + password.length
  );

  let response = null;
  try {
    response = await fetch(apiLocation + "token", {
      method: 'POST',
      body: JSON.stringify({ "username": username, "password": password }),
      headers: {'Content-Type': 'application/json'}
    });
  } catch (e) {
    statusElement.innerHTML =
      "Failed to get token due to an error. Check console for more info.";
    window.localStorage.
    return;
  }
  let data = await response.json();
  if (!response.ok) {
    statusElement.innerHTML =
      "Got HTTP code " +
      response.status +
      ". Reason: " +
      (data["error"] ? data["error"] : "unknown");
    return;
  }
  console.log(data);
  statusElement.innerHTML = "logged in successfully!";
  token = 'Bearer ' + data['token'];
  await populateLinks();
};


async function populateLinks() {
  const linksTableElement = document.getElementById("links-table");
  const statusElement = document.getElementById("login-status");
  const loginElement = document.getElementById("login");
  const panelElement = document.getElementById("panel");
  console.log(token);
  let response = null;
  try {
    response = await fetch(apiLocation + "list", {
      headers: { "Authorization": token },
    });
  } catch (e) {
    statusElement.innerHTML =
      "Failed to get links due to an error. Check console for more info.";
    return;
  }
  let data = await response.json();
  if (!response.ok) {
    statusElement.innerHTML =
      "Got HTTP code " +
      response.status +
      ". Reason: " +
      (data["error"] ? data["error"] : "unknown");
    return;
  }
  const linkArray = data["links"];
  let idx = 0;
  for (const link in linkArray) {
    let linkElement = document.createElement("label");
    linkElement.innerText = link;

    let submitButton = document.createElement("button");
    submitButton.innerHTML = "Update";
    submitButton.addEventListener("click", function () { updateExisting(submitButton) });
    submitButton.setAttribute("class", "update-button");

    let destElement = document.createElement("input");
    destElement.type = "url";
    destElement.value = linkArray[link];
    destElement.placeholder = "Destination";
    destElement.setAttribute("size", "50");
    async function updateExistingKeyPressHandler(event) { if (event.key === "Enter") await updateExisting(submitButton); }
    destElement.addEventListener("keypress", updateExistingKeyPressHandler);

    let deleteButton = document.createElement("button");
    deleteButton.innerHTML = "Delete";
    deleteButton.addEventListener("click", function () { deleteExisting(deleteButton) });
    deleteButton.setAttribute("class", "delete-button");

    let qrCodeButton = document.createElement("button");
    qrCodeButton.innerHTML = "Get QR code";
    qrCodeButton.addEventListener("click", function () { openQrCode(qrCodeButton) });
    qrCodeButton.setAttribute("class", "qr-button");

    let newRowElement = linksTableElement.insertRow(idx + 1);
    newRowElement.setAttribute("class", "link-row");

    let linkCellElement = newRowElement.insertCell();
    linkCellElement.appendChild(linkElement);

    let destCellElement = newRowElement.insertCell();
    destCellElement.appendChild(destElement);

    let actionsCellElement = newRowElement.insertCell();
    actionsCellElement.appendChild(submitButton);
    actionsCellElement.appendChild(deleteButton);
    actionsCellElement.appendChild(qrCodeButton);
    idx++;
  }
  totalUrls = idx;

  loginElement.style.display = "none";
  sortTable();
  panelElement.style.display = null;
}

async function addNew() {
  const linksTableElement = document.getElementById("links-table");
  const srcElement = document.getElementById("new-link-src");
  const dstElement = document.getElementById("new-link-dst");
  const src = srcElement.value;
  const dst = dstElement.value;

  let response = await fetch(apiLocation + "add", {
    headers: {
      "Authorization": token,
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ link: src, destination: dst }),
    method: "PUT",
  });
  if (!response.ok) {
    let data = await response.json();
    alert(
      "Got HTTP code " +
      response.status +
      ". Reason: " +
      (data["error"] ? data["error"] : "unknown")
    );
    return;
  }

  srcElement.value = "";
  dstElement.value = "";

  let linkElement = document.createElement("label");
  linkElement.innerText = "/" + src;

  let submitButton = document.createElement("button");
  submitButton.innerHTML = "Update";
  submitButton.addEventListener("click", function () { updateExisting(submitButton) });
  submitButton.setAttribute("class", "update-button");

  let destElement = document.createElement("input");
  destElement.type = "url";
  destElement.value = dst;
  destElement.placeholder = "Destination";
  destElement.setAttribute("size", "50");
  async function updateExistingKeyPressHandler(event) { if (event.key === "Enter") await updateExisting(submitButton); }
  destElement.addEventListener("keypress", updateExistingKeyPressHandler);

  let deleteButton = document.createElement("button");
  deleteButton.innerHTML = "Delete";
  deleteButton.addEventListener("click", function () { deleteExisting(deleteButton) });
  deleteButton.setAttribute("class", "delete-button");

  let qrCodeButton = document.createElement("button");
  qrCodeButton.innerHTML = "Get QR code";
  qrCodeButton.addEventListener("click", function () { openQrCode(qrCodeButton) });
  qrCodeButton.setAttribute("class", "qr-button");

  let newRowElement = linksTableElement.insertRow(totalUrls + 1);
  newRowElement.setAttribute("class", "link-row");

  let linkCellElement = newRowElement.insertCell();
  linkCellElement.appendChild(linkElement);

  let destCellElement = newRowElement.insertCell();
  destCellElement.appendChild(destElement);

  let actionsCellElement = newRowElement.insertCell();
  actionsCellElement.appendChild(submitButton);
  actionsCellElement.appendChild(deleteButton);
  actionsCellElement.appendChild(qrCodeButton);

  totalUrls++;
  sortTable();
}

async function updateExisting(element) {
  let cells = element.parentElement.parentElement.children;
  let src = cells[0].innerText;
  let dst = cells[1].firstElementChild.value;
  let response = await fetch(
    apiLocation + "edit/" + encodeURIComponent(src),
    {
      headers: {
        "Authorization": token,
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ destination: dst }),
      method: "PATCH",
    }
  );
  if (!response.ok) {
    let data = await response.json();
    alert(
      "Got HTTP code " +
      response.status +
      ". Reason: " +
      (data["error"] ? data["error"] : "unknown")
    );
  } else {
    alert("Updated URL successfully.");
  }
}

async function deleteExisting(element) {
  let linksTable = document.getElementById("links-table");
  let row = element.parentElement.parentElement;
  let src = row.children[0].innerText;
  let response = await fetch(
    apiLocation + "delete/" + encodeURIComponent(src),
    {
      headers: {
        "Authorization": token,
        "Content-Type": "application/json",
      },
      method: "DELETE",
    }
  );
  if (!response.ok) {
    let data = await response.json();
    alert(
      "Got HTTP code " +
      response.status +
      ". Reason: " +
      (data["error"] ? data["error"] : "unknown")
    );
  } else {
    linksTable.deleteRow(row.rowIndex);
    totalUrls--;
  }
}

async function openQrCode(element) {
  let linksTable = document.getElementById("links-table");
  let row = element.parentElement.parentElement;
  let src = row.children[1].firstChild.value;
  let response = await fetch(
    apiLocation + "qr",
    {
      headers: {
        "Authorization": token,
        "Content-Type": "application/json",
      },
      method: "POST",
      body: JSON.stringify({ "destination": src })
    }
  );
  if (!response.ok) {
    let data = await response.json();
    alert(`Got HTTP code ${response.status}. Reason: ${data["error"] ? data["error"] : "unknown"}`);
  }
  (function (root, factory) { typeof define == "function" && define.amd ? define([], factory) : typeof exports == "object" ? module.exports = factory() : root.download = factory() })(this, function () { return function download(data, strFileName, strMimeType) { var self = window, defaultMime = "application/octet-stream", mimeType = strMimeType || defaultMime, payload = data, url = !strFileName && !strMimeType && payload, anchor = document.createElement("a"), toString = function (a) { return String(a) }, myBlob = self.Blob || self.MozBlob || self.WebKitBlob || toString, fileName = strFileName || "download", blob, reader; myBlob = myBlob.call ? myBlob.bind(self) : Blob, String(this) === "true" && (payload = [payload, mimeType], mimeType = payload[0], payload = payload[1]); if (url && url.length < 2048) { fileName = url.split("/").pop().split("?")[0], anchor.href = url; if (anchor.href.indexOf(url) !== -1) { var ajax = new XMLHttpRequest; return ajax.open("GET", url, !0), ajax.responseType = "blob", ajax.onload = function (e) { download(e.target.response, fileName, defaultMime) }, setTimeout(function () { ajax.send() }, 0), ajax } } if (/^data:([\w+-]+\/[\w+.-]+)?[,;]/.test(payload)) { if (!(payload.length > 2096103.424 && myBlob !== toString)) return navigator.msSaveBlob ? navigator.msSaveBlob(dataUrlToBlob(payload), fileName) : saver(payload); payload = dataUrlToBlob(payload), mimeType = payload.type || defaultMime } else if (/([\x80-\xff])/.test(payload)) { var i = 0, tempUiArr = new Uint8Array(payload.length), mx = tempUiArr.length; for (i; i < mx; ++i)tempUiArr[i] = payload.charCodeAt(i); payload = new myBlob([tempUiArr], { type: mimeType }) } blob = payload instanceof myBlob ? payload : new myBlob([payload], { type: mimeType }); function dataUrlToBlob(strUrl) { var parts = strUrl.split(/[:;,]/), type = parts[1], indexDecoder = strUrl.indexOf("charset") > 0 ? 3 : 2, decoder = parts[indexDecoder] == "base64" ? atob : decodeURIComponent, binData = decoder(parts.pop()), mx = binData.length, i = 0, uiArr = new Uint8Array(mx); for (i; i < mx; ++i)uiArr[i] = binData.charCodeAt(i); return new myBlob([uiArr], { type: type }) } function saver(url, winMode) { if ("download" in anchor) return anchor.href = url, anchor.setAttribute("download", fileName), anchor.className = "download-js-link", anchor.innerHTML = "downloading...", anchor.style.display = "none", anchor.addEventListener("click", function (e) { e.stopPropagation(), this.removeEventListener("click", arguments.callee) }), document.body.appendChild(anchor), setTimeout(function () { anchor.click(), document.body.removeChild(anchor), winMode === !0 && setTimeout(function () { self.URL.revokeObjectURL(anchor.href) }, 250) }, 66), !0; if (/(Version)\/(\d+)\.(\d+)(?:\.(\d+))?.*Safari\//.test(navigator.userAgent)) return /^data:/.test(url) && (url = "data:" + url.replace(/^data:([\w\/\-\+]+)/, defaultMime)), window.open(url) || confirm("Displaying New Document\n\nUse Save As... to download, then click back to return to this page.") && (location.href = url), !0; var f = document.createElement("iframe"); document.body.appendChild(f), !winMode && /^data:/.test(url) && (url = "data:" + url.replace(/^data:([\w\/\-\+]+)/, defaultMime)), f.src = url, setTimeout(function () { document.body.removeChild(f) }, 333) } if (navigator.msSaveBlob) return navigator.msSaveBlob(blob, fileName); if (self.URL) saver(self.URL.createObjectURL(blob), !0); else { if (typeof blob == "string" || blob.constructor === toString) try { return saver("data:" + mimeType + ";base64," + self.btoa(blob)) } catch (y) { return saver("data:" + mimeType + "," + encodeURIComponent(blob)) } reader = new FileReader, reader.onload = function (e) { saver(this.result) }, reader.readAsDataURL(blob) } return !0 } });
  download(await response.blob(), "qr_code.bmp")
}
// register all event listeners
document.body.addEventListener("load", onLoad);
document.getElementById("login-button").addEventListener("click", login);
document.getElementById("add-button").addEventListener("click", addNew);
document.getElementById("logout-button").addEventListener("click", logout);
async function loginKeyPressHandler(event) { if (event.key === "Enter") await login(); }
document.getElementById("password-input").addEventListener("keypress", loginKeyPressHandler);
document.getElementById("username-input").addEventListener("keypress", loginKeyPressHandler);
async function newLinkKeyPressHandler(event) { if (event.key === "Enter") await addNew(); }
document.getElementById("new-link-src").addEventListener("keypress", newLinkKeyPressHandler);
document.getElementById("new-link-dst").addEventListener("keypress", newLinkKeyPressHandler);
function sortTable() {
var table, rows, switching, i, x, y, shouldSwitch;
table = document.getElementById("links-table");
switching = true;
while (switching) {
  switching = false;
  rows = table.rows;
  for (i = 1; i < (rows.length - 2); i++) {
    shouldSwitch = false;
    x = rows[i].getElementsByTagName("TD")[0];
    y = rows[i + 1].getElementsByTagName("TD")[0];
    if (x.innerHTML.toLowerCase() > y.innerHTML.toLowerCase()) {
      shouldSwitch = true;
      break;
    }
  }
  if (shouldSwitch) {
    rows[i].parentNode.insertBefore(rows[i + 1], rows[i]);
    switching = true;
  }
}
}