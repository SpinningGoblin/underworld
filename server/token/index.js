window.onload = function () {
  const tokenWrapElement = document.getElementById("api-token-wrap");
  const tokenElement = document.getElementById("api-token");
  const cookieValue = document.cookie
      .split("; ")
      .find((row) => row.startsWith("underworldApiToken="))
      ?.split("=")[1];
  if (tokenElement && cookieValue) {
    tokenElement.innerText = cookieValue;
    tokenWrapElement.hidden = false;
  } else {
    tokenWrapElement.hidden = true;
  }
};
