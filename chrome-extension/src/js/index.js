window.onload = function() {
  fetch("https://portfolio.artell.life/api/ichibanchi/art")
    .then(res => res.json())
    .then(json => {
      var imgEle = document.getElementById("works");
      var titleEle = document.getElementById("title");
      var artistEle = document.getElementById("artist");
      var materialsEle = document.getElementById("materials");
      var sizeEle = document.getElementById("size");

      // caption情報の設定
      titleEle.textContent = json.artTitle;
      artistEle.textContent = json.artistName;
      materialsEle.textContent = json.artMaterials;
      if (json.artSize === null) {
        sizeEle.textContent = "";
      } else {
        sizeEle.textContent =
          json.artSize[0] + " x " + json.artSize[1] + " mm ";
      }

      // ポートフォリオへのリンクの設定
      document.getElementById("link").setAttribute("href", json.portfolioLink);

      // 画像の設定
      imgEle.setAttribute("src", json.imageUrl);
      imgEle.onload = function() {
        imgEle.classList.add("show");
      };
    });
};
