import datepicker from "js-datepicker";
import Selectr from "mobius1-selectr";
      
      var day = document.getElementById('datepicker');
      var hour = document.getElementById('hour');
      var minute = document.getElementById('minute');
      var second = document.getElementById('second');

      var picker = datepicker(day);
      new Selectr(hour, {});
      new Selectr(minute, {});
      new Selectr(second, {});

      var now = new Date();
      day.value = now.toLocaleString('en-US', {
        weekday: 'short',
        month: 'short',
        day: 'numeric',
        year: 'numeric'
      }).replace(',', '').replace(',', '');

      function go() {
        const eventName = document.getElementById('eventName').value;
        const date = new Date(day.value);
        date.setHours(date.getHours() + hour.value);
        date.setMinutes(date.getMinutes() + minute.value);
        date.setSeconds(date.getSeconds() + second.value);

        var modifiedEventName = "";
        for (const char of eventName) {
          if (char === ' ') {
            modifiedEventName += '-';
          }
          else if (char === '-') {
            modifiedEventName += ' ';
          }
          else {
            modifiedEventName += char;
          }
        }

        window.location.href += "event/" + encodeURIComponent(modifiedEventName) + "?time=" + date.getTime();
      }