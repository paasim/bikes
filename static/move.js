function handleSwipe(elem) {
  elem.addEventListener("touchstart", (event) => {
    const touch = event.changedTouches[0];
    localStorage.setItem('start-id', touch.identifier);
    localStorage.setItem('start-x', touch.clientX);
    localStorage.setItem('start-y', touch.clientY);
  });

  elem.addEventListener("touchmove", (event) => event.preventDefault());

  elem.addEventListener("touchend", (event) => {
    const touch = event.changedTouches[0];
    if (String(touch.identifier) === localStorage.getItem('start-id')) {
      const dx = touch.clientX - Number(localStorage.getItem('start-x'));
      const dy = touch.clientY - Number(localStorage.getItem('start-y'));
      if (Math.max(Math.abs(dx), Math.abs(dy)) < 20) return;

      const params = new URLSearchParams(window.location.search);
      if (Math.abs(dx) > Math.abs(dy)) {
        params.set('dx', Number(params.get('dx')) - Math.sign(dx))
      } else {
        params.set('dy', Number(params.get('dy')) - Math.sign(dy))
      }
      window.location.search = params.toString();
    }
  });
}

function move() {
  handleSwipe(document.querySelector('.img-container'));
}

window.onload = move;
