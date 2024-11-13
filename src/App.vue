<template>
  <div class="app-container">
    <div data-tauri-drag-region class="titlebar">
      <div class="titlebar-button" id="titlebar-minimize" @click="minimizeWindow">
        <i class="fa-solid fa-window-minimize"></i>
      </div>
      <div class="titlebar-button" id="titlebar-maximize" @click="maximizeWindow">
        <i class="fa-solid fa-window-maximize"></i>
      </div>
      <div class="titlebar-button" id="titlebar-close" @click="closeWindow">
        <i class="fa-solid fa-xmark"></i>      
      </div>
    </div>
    <!-- Sidebar -->
    <aside
      :class="['sidebar', { 'sidebar--extended': isSidebarExtended }]"
      @mouseover="extendSidebar"
      @mouseleave="collapseSidebar"
    >
      <nav>
        <ul>
          <!-- Persona List item with icon and animated label -->
          <li @click="setActiveComponent('personaList')">
            <i class="fa-solid fa-plus"></i>
            <Transition name="slide" mode="out-in">
              <template v-if="isSidebarExtended">
                <span class="label">PersonaList</span>
              </template>
            </Transition>
          </li>

          <!-- App Profile item with icon and animated label -->
          <li @click="setActiveComponent('appProfile')">
            <i class="fa-solid fa-user"></i>
            <Transition name="slide">
              <template v-if="isSidebarExtended">
                <span class="label">App Profile</span>
              </template>
            </Transition>
          </li>

          <!-- App Settings item with icon and animated label -->
          <li @click="setActiveComponent('appSettings')">
            <i class="fa-solid fa-cogs"></i>
            <Transition name="slide">
              <template v-if="isSidebarExtended">
                <span class="label">App Settings</span>
              </template>
            </Transition>
          </li>
        </ul>
      </nav>
    </aside>

    <!-- Main Content Area -->
    <main class="main-content">
      <component :is="activeComponent" />
    </main>
  </div>
</template>

<script>
import personaList from "/src/components/personaList.vue";
import appProfile from "/src/components/appProfile.vue";
import appSettings from "/src/components/appSettings.vue";
import { getCurrentWindow } from '@tauri-apps/api/window';
import "@fortawesome/fontawesome-free/css/all.css";

export default {
  data() {
    return {
      isSidebarExtended: false,
      activeComponent: "personaList", // Default component
      window: getCurrentWindow()
    };
  },
  components: {
    personaList,
    appProfile,
    appSettings,
  },
  methods: {
    extendSidebar() {
      this.isSidebarExtended = true;
    },
    collapseSidebar() {
      this.isSidebarExtended = false;
    },
    setActiveComponent(componentName) {
      this.activeComponent = componentName;
    },
    minimizeWindow() {
      this.window.minimize();
    },
    maximizeWindow() {
      this.window.maximize();
    },
    closeWindow() {
      this.window.close();
    },
  },
};
</script>

<style scoped>
.app-container {
  display: flex;
  min-height: 100vh;
  min-width: 100vw;
  background: radial-gradient(
    circle at top left,
    rgba(43, 47, 54, 0.95),
    rgba(27, 30, 35, 0.9) 70%,
    rgba(17, 20, 23, 1) 100%
  );
  backdrop-filter: blur(50px);
}

/* Sidebar */
.sidebar {
  width: 60px;
  transition: width 0.3s ease;
  height: 100vh;
  display: flex;
  flex-direction: column;
  align-items: center;
  background: rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(50px);
  border-right: 1px solid rgba(255, 255, 255, 0.5);
  overflow: hidden;
}

.sidebar--extended {
  width: 200px;
}

/* Sidebar Navigation */
.sidebar nav ul {
  list-style: none;
  padding: 0;
}

.sidebar nav ul li {
  padding: 15px;
  cursor: pointer;
  color: white;
  transition: background 0.3s ease;
  display: flex;
  align-items: center;
}

.fa-solid {
  font-size: 20px;
}

.sidebar nav ul li:hover {
  background: rgba(255, 255, 255, 0.1);
}

/* Label Slide Animation */
.sidebar .label {
  display: inline-block;
  opacity: 0;
  transform: translateX(-20px);
  transition: opacity 0.3s ease, transform 0.3s ease;
  margin-left: 10px;
  text-wrap: nowrap;
}

/* When sidebar is extended, label slides in */
.sidebar--extended .label {
  opacity: 1;
  transform: translateX(0);
}

/* Main Content */
.main-content {
  flex: 1;
  padding: 20px;
  color: white;
  display: flex;
  justify-content: center;
  align-items: center;
}

/* Slide Transition */
.slide-enter-active,
.slide-leave-active {
  transition: opacity 0.3s ease, transform 0.3s ease;
}

.slide-enter, .slide-leave-to /* .slide-leave-active in <2.1.8 */ {
  opacity: 0;
  transform: translateX(-20px);
}
</style>

<style>
.titlebar {
  height: 30px;
  user-select: none;
  display: flex;
  justify-content: flex-end;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
}
.titlebar-button {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 30px;
  height: 30px;
  user-select: none;
  -webkit-user-select: none;
}
.titlebar-button:hover {
  background: rgba(87, 39, 39, 0.1);
}
.titlebar .fa-solid{
  color: #d1d1d1;
}
</style>
