<template>
  <skeleton-theme color="#0F0F0F" highlight="#151515">
    <div
    @click="hidden = !hidden"
    class="server"
    :class="{hidden}"
    :style="{
        'background-image': `url(${server.icon})`
    }">
    <img :src="server.icon">
    <div>
      <h1 class="has-text-white is-uppercase is-size-7">{{ server.name }}</h1>
        <skeleton width="100px" height="15px" loading="true">
          {{ color }}
        </skeleton>
        <skeleton width="150px" height="15px" loading="true">
          {{ color }}
        </skeleton>
        <skeleton width="130px" height="15px" loading="true">
          {{ color }}
        </skeleton>
    </div>
    <div class="graph">
      <skeleton width="100%" height="100%" loading="true">
        {{ color }}
      </skeleton>
    </div>
    </div>
  </skeleton-theme>
</template>

<script>
import * as Vibrant from 'node-vibrant'
import { Skeleton, SkeletonTheme } from 'vue-loading-skeleton'

export default {
  props: ['server'],
  components: { Skeleton, SkeletonTheme },
  data () {
    return {
      hidden: false,
      color: ''
    }
  },
  created () {
    if (process.server) {
      return
    }

    setTimeout(() => {
      Vibrant.from(this.server.icon)
        .getPalette((_, palette) => {
          this.color = palette.LightVibrant.getHex()
        })
    }, 1000)
  }
}
</script>

<style>
.server {
  padding: 2rem;
  background-position: center;
  background-size: cover;
  background-repeat: no-repeat;
  border-radius: 4px;
  position: relative;
  overflow: hidden;
  transition: opacity 0.3s;
  cursor: pointer;
  display: flex;
  user-select: none;
  align-items: center;
}

.server.hidden {
    opacity: 0.5;
}

.server > * {
    position: relative;
    z-index: 99;
}

.server::after {
  position: absolute;
  left: 0;
  top: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.69);
  content: '';
}

.server > div > span {
  font-size: 12px;
}

img {
  width: 78px;
  height: 78px;
  margin-right: 1rem;
  image-rendering: pixelated;
}

.server > div:first-of-type {
  display: flex;
  align-items: flex-start;
  align-content: flex-start;
  flex-direction: column;
}

h1 {
  letter-spacing: 4px;
  font-weight: 800 !important;
}

.graph {
  flex: 1;
  display: flex;
  justify-content: flex-end;
  align-self: stretch;
  margin-left: 1rem;
}

.graph > span {
  display: block;
  flex: 1;
  height: 100%;
}
</style>
