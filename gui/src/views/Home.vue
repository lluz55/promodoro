<template>
<div class="home">
  <Clock :timerValue="timerValue" :play="playing" @completed="onActionCompleted" />
  <div class="wrapper">
    <button @click="play"><i :class="[playing ? 'icon-pause' : 'icon-play', 'btn-play']"></i></button>
    <button @click="resetTimer" :class="[stoped ? 'disabled': '', 'btn-reset']">Reset</button>

    <div hidden>
      <button @click="newTimer">New Timer</button>
      <input v-model="newTime" min="01:00" type="time" />
    </div>
  </div>
</div>
</template>

<script lang="ts">
import {
  defineComponent,
  onMounted,
  ref,
} from 'vue';
import Clock from '@/components/Clock.vue';

export default defineComponent({
  name: 'Home',
  components: {
    Clock,
  },
  setup() {
    const defaultTimer = "25:00";

    let timerValue = ref(defaultTimer);
    let newTime = ref("00:02:00");
    let playing = ref(false);
    let stoped = ref(true);

    let play = () => {
      stoped.value = false;
      playing.value = !playing.value;
    }

    let onActionCompleted = () => {
      console.log("action completed")
      stoped.value = true;
      playing.value = false;
    }
    let newTimer = () => {
      if (newTime.value.length > 5) {
        newTime.value = newTime.value.substring(0, 5);
      }
      timerValue.value = newTime.value;
    }

    let resetTimer = () => {
      if (stoped.value) return
      stoped.value = true;
      timerValue.value = "-"
      setTimeout(() => timerValue.value = defaultTimer, 1)
    }

    return {
      play,
      playing,
      stoped,
      newTime,
      onActionCompleted,
      timerValue,
      newTimer,
      resetTimer
    }
  }
});
</script>

<style lang="scss" scoped>
$wrapper-margin-left: 20px;
$wrapper-margin-bottom: 100px;

.home {
  min-height: 100% !important;
  height: 100%;
  display: flex;
  flex-grow: 1;
  justify-content: space-around;
  flex-direction: column;
}

.wrapper {
  display: flex;
  justify-content: center;
  margin-bottom: $wrapper-margin-bottom;
  flex-direction: column;
  align-content: center;
}

.wrapper>* {
  //   margin-right: $wrapper-margin-left;
  margin-top: 50px;
}

button {
  border: 0;
  padding: 0;
  line-height: 1;
  outline: none;
  //background-color: transparent;
  max-width: 100px;
  margin: 0 auto;
  cursor: pointer;
}

button:active {
  opacity: .9;
}

.btn-play {
  background-color: #2F71D3;
  padding: 16px;
  border-radius: 50%;
  font-size: 1.5em;
}

.btn-reset {
  animation: .5s ease forwards fadein;
  padding: 10px 26px;
  border-radius: 8px;
  text-transform: uppercase;
  background-color: transparent;
  letter-spacing: 4px;
  color: #ccc;
  font-size: 1.2em;
  font-weight: 600;
  padding-left: 16px;
  cursor: pointer;
}

.btn-reset:hover {
  color: #999;
}

.disabled {
  cursor: default;
  animation: .5s ease forwards fadeout;
}

@keyframes fadein {
  100% {
    opacity: 1;
  }

  50% {
    opacity: .5;
  }

  0% {
    opacity: 0;
  }
}

@keyframes fadeout {
  0% {
    opacity: 1;
  }

  50% {
    opacity: .5;
  }

  100% {
    opacity: 0;
  }
}

[class^="icon-"] {
  color: #f5f5f5;
}
</style>
