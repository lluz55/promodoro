<template>
<div id="app">
  <div id="nav">

  </div>
  <router-view />
</div>
</template>

<style lang="scss">
@import "~fontsource-quicksand/index.css";

.clearfix {
  *zoom: 1;
}

.clearfix:before,
.clearfix:after {
  display: table;
  content: "";
  line-height: 0;
}

.clearfix:after {
  clear: both;
}

body,
ui,
li,
p {
  list-style: none;
  margin: 0;
  padding: 0;
}

html,
body,
#app {
  height: 90vh;
}

* {

  font-family: "Quicksand";
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

#app {
  font-family: "Quicksand";
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #202020;
}

#nav {
  padding: 30px;

  a {
    font-weight: bold;
    color: #2c3e50;

    &.router-link-exact-active {
      color: #42b983;
    }
  }
}
</style>
