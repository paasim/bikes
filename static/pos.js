function redirectLatLon(lat, lon) {
  window.location.replace(`?lat=${lat}&lon=${lon}`);
}

function getLocation() {
  if (navigator.geolocation) {
    navigator.geolocation.getCurrentPosition((pos) => {
      redirectLatLon(pos.coords.latitude, pos.coords.longitude);
    });
  }
}

window.onload = getLocation;
