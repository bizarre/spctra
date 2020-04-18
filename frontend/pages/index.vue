<template>
  <div id="app" class="section">
    <distribution-bar />
    <div class="columns">
      <sidebar />
    </div>
  </div>
</template>

<script>
import Sidebar from '~/components/Sidebar.vue'
import DistributionBar from '~/components/DistributionBar.vue'
export default {
  components: {
    Sidebar, DistributionBar
  },

  asyncData ({ $axios, route, error, redirect, store }) {
    return $axios.get('/api/servers').then((response) => {
      const servers = response.data

      return { servers }
    }).catch((e) => {
      error({ statusCode: 404, message: 'Could not load servers.' })
    })
  },

  created () {
    if (process.server) {
      return
    }

    this.$store.commit('setServers', this.servers)
  }
}
</script>

<style>
#app {
  height: 100vh;
  padding-top: 12px;
  padding-left: 12px;
  padding-right: 12px;
  padding-bottom: 0px;
  /*background: linear-gradient(90deg,#133630 0%, #133236 100%);*/
  background: black;
}

.columns {
  height: 100%;
}
</style>
