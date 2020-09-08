<template>
<div class="clock">
  <div class="wrapper">
    <strong>{{minutes}}</strong>
    <span>:{{compSec}}</span>
  </div>
</div>
</template>

<script lang="ts">
import {
  computed,
  defineComponent,
  ref,
  Ref,
  watch,
  watchEffect,
} from 'vue'
export default defineComponent({
  props: {
    timerValue: {
      required: true,
      type: String,
    },
    play: {
      required: true,
      type: Boolean,
    }
  },
  setup(props, context) {
    let minutes: Ref < number > = ref(25);
    let seconds: Ref < number > = ref(0);
    let timer: Ref < number > = ref(0);

    let work_time: string = "00:10";

    let compSec = computed(() => {
      if (seconds.value < 10) {
        return "0" + seconds.value;
      }
      return seconds.value;
    });

    watch(() => props.timerValue, (o, n) => {
      if (props.timerValue == "-") {
        return
      }
      context.emit("completed", true)

      let parts = props.timerValue.split(":");
      let min = parseInt(parts[0]);
      let sec = parseInt(parts[1]);
      if (isNaN(min) || isNaN(sec)) return;
      if (sec > 59) {
        sec = 0;
        min++;
      }
      minutes.value = min;
      seconds.value = sec;
    });

    let check_time_finished = () => {
      if (seconds.value == 0 && minutes.value == 0) {
        context.emit("completed", true)
        clearInterval(timer.value)
      }
    };

    watch(() => props.play, (o, n) => {
      if (props.play) {
        check_time_finished()
        timer.value = setInterval(() => {
          seconds.value--;
          check_time_finished()
          if (seconds.value < 0) {
            seconds.value = 59;
            minutes.value--;
          }
        }, 1000)
      } else {
        clearInterval(timer.value)
      }
    })

    return {
      minutes,
      compSec,
    }
  }
})
</script>

<style lang="scss" scoped>
.clock {
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.wrapper {
  justify-content: center;
  display: flex;
  font-size: 4em;
  font-weight: bolder;

  >span {
    font-weight: lighter;
    color: #909090;
  }
}
</style>
