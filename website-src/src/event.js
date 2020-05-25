var time = parseInt("{{ unix_epoch_time_offset }}");
var countdown = document.getElementById("time");

displayEventTime();
setInterval(displayEventTime, 1000);

function displayEventTime() {
  var now = new Date().getTime();
  var difference = time - now;

  countdown.innerText = toDisplayString(difference);
}

function toDisplayString(time) {
  var startTime = time;
  var result = "";

  time -= time % 1000;
  time /= 1000;

  var seconds = time % 60;
  time -= seconds;
  time /= 60;

  var minutes = time % 60;
  time -= minutes;
  time /= 60;

  var hours = time % 24;
  time -= hours;
  time /= 24;

  var days = time;

  var negativePrefix = startTime < 0 ? '-' : '';

  return days + " days, " + negativePrefix + padNumber(hours) + ":" + padNumber(minutes) + ":" + padNumber(seconds) + " left.";
}

function padNumber(number) {
  if (number < 0) {
    return padNumber(number * -1);
  }

  return number.toString().padStart(2, '0');
}