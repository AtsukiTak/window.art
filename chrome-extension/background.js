chrome.runtime.onMessage.addListener(function (req, sender, sendResponse) {
  if (req === "fetchCurrentArt") {
    var url = "https://artell.herokuapp.com/api/v1/user/get_current_url";
    fetch(url)
      .then(res => res.json())
      .then(res => sendResponse(res))
      .catch()
    return true;
  }
});
