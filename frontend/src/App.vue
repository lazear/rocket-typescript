<template>
  <v-app dark>
    <v-toolbar app>
      <v-toolbar-title class="headline text-uppercase">
        <span>Rust</span>
        <span class="font-weight-light">+Typescript</span>
      </v-toolbar-title>
      <v-btn
        flat
        @click="$router.push({name: 'home'})"
        target="_blank"
      >
        <span class="mr-2">Home</span>
      </v-btn>
      <v-btn
        flat
        @click="$router.push('about')"
        target="_blank"
      >
        <span class="mr-2">About</span>
      </v-btn>
      <v-spacer></v-spacer>
    </v-toolbar>

    <v-content>
      <router-view></router-view>
    </v-content>
  </v-app>
</template>

<script>
import HelloWorld from './components/HelloWorld';
import Counter from './components/Counter';

export default {
  name: 'App',
  components: {
    HelloWorld,
    Counter,
  },
  data () {
    return {
      //
    }
  }
}
</script>
