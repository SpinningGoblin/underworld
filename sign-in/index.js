window.onload = function () {
  if (window.location.hash === "#auth_failure") {
    window.location.hash = "";
    alert("Authentication failed");
  }
};
