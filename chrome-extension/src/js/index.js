window.onload = function() {
  fetch("https://artell.herokuapp.com/api/v1/user/get_current_art")
    .then(res => res.json())
    .then(json => {
      var imgEle = document.getElementById("works");
      var captionEle = document.getElementById("caption");
      var titleEle = document.getElementById("title");
      var artistEle = document.getElementById("artist");

      // caption情報の設定
      titleEle.textContent = json.artTitle;
      artistEle.textContent = json.artistName;

      // ポートフォリオへのリンクの設定
      var encodedArtistName = encodeURI(json.artistName);
      var portoflioUrl = `https://portfolio.artell.life/${encodedArtistName}/${json.portfolioId}/`;
      document.getElementById("link").setAttribute("href", portoflioUrl);

      // 画像の設定
      imgEle.setAttribute("src", json.imageUrl);
      imgEle.onload = function() {
        imgEle.classList.add("show");
        // imgのwidthに合わせて、キャプションのwidthを決定する
        captionEle.style.width = imgEle.width + "px";
      };
    });
};
