window.onload = function() {
  var img = document.getElementById("works");
  var caption = document.getElementById("caption");

  // imgのwidthに合わせて、キャプションのwidthを決定する
  caption.style.width = img.width + "px";

  fetch("https://artell.herokuapp.com/api/v1/user/get_current_art")
    .then(res => res.json())
    .then(res => {
      document.getElementById("works").setAttribute("src", res.imageUrl);
    });
};
