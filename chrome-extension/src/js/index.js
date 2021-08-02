window.onload = function() {
  fetch("https://portfolio.artell.life/api/ichibanchi/art")
    .then(res => res.json())
    .then(json => {

      // 画像の設定
      var imgEle = document.querySelector(".image");
      imgEle.setAttribute("src", json.imageUrl);
      imgEle.onload = function() {
        // 画像の位置を設定
        var imgWidth = imgEle.clientWidth;
        var viewWidth = document.body.clientWidth;
        imgEle.style.marginLeft = `${(viewWidth - imgWidth) / 4}px`;

        // linkの設定
        let linkElement = document.querySelector(".link");
        if (imgWidth > viewWidth * 0.7) {
          linkElement.classList.add("vertical");
        }
        linkElement.setAttribute("href", json.portfolioLink);
        linkElement.classList.add("show");

        // 画像とlinkの表示
        var itemsEle = document.querySelector(".items");
        itemsEle.classList.add("show");
      };
    });
};
