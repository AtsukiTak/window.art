window.onload = function() {

  fetch("https://artell.herokuapp.com/api/v1/user/get_current_art")
    .then(res => res.json())
    .then(json => {
      var img = document.getElementById("works");
      var caption = document.getElementById("caption");

      img.setAttribute("src", json.imageUrl);
      img.onload = function() {
        img.classList.add("show");
        // imgのwidthに合わせて、キャプションのwidthを決定する
        caption.style.width = img.width + "px";
      };

    });
};
