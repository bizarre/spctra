export const state = () => ({
  servers: []
})

export const getters = {
  getServers (state) {
    return state.servers
  }
}

export const mutations = {
  setServers (state, servers) {
    state.servers = servers
  }
}
