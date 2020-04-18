<template>
  <div id="sidebar" class="column is-5">
    <server v-for="server in servers" :key="server.id" :server="server" />
  </div>
</template>

<script>
import { mapGetters } from 'vuex'
import Server from './Server.vue'

export default {
  components: { Server },
  computed: {
    ...mapGetters({
      servers: 'getServers'
    })
  }
}
</script>

<style>
#sidebar {
    /*background: linear-gradient(90deg, #1D4C56 0%, #1D5648 100%);*/
    background: #0B0B0B;
    height: 100%;
    margin-bottom: 0;
    display: flex;
    flex-direction: column;
    padding: 1rem;
}

#sidebar > div {
  margin-bottom: 1rem;
}

#sidebar > div:first-of-type {
  margin-top: 5px;
}
</style>
