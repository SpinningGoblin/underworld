window.onload = function() {
  const cookieValues = document.cookie
      .split("; ")
      .find((row) => row.startsWith("underworldCallback="))
      ?.split("=");

    const url = cookieValues.slice(1, cookieValues.length).join("=");

  if (url) {
    window.location.href = url;
  }
}
