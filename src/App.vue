<template>
  <div class="app-container">
    <!-- Titlebar -->
    <div data-tauri-drag-region class="titlebar">
      <div
        class="titlebar-button"
        id="titlebar-minimize"
        @click="minimizeWindow"
      >
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 11 11">
          <path fill="#fff" d="M11,4.9v1.1H0V4.399h11z" />
        </svg>
      </div>
      <div
        class="titlebar-button"
        id="titlebar-maximize"
        @click="maximizeWindow"
      >
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 11 11">
          <path
            fill="#fff"
            d="M0,1.7v7.6C0,10.2,0.8,11,1.7,11h7.6c0.9,0,1.7-0.8,1.7-1.7V1.7C11,0.8,10.2,0,9.3,0H1.7C0.8,0,0,0.8,0,1.7z M8.8,9.9H2.2c-0.6,0-1.1-0.5-1.1-1.1V2.2c0-0.6,0.5-1.1,1.1-1.1h6.7c0.6,0,1.1,0.5,1.1,1.1v6.7C9.9,9.4,9.4,9.9,8.8,9.9z"
          />
        </svg>
      </div>
      <div class="titlebar-button" id="titlebar-close" @click="closeWindow">
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 11 11">
          <path
            fill="#fff"
            d="M6.279 5.5L11 10.221l-.779.779L5.5 6.279.779 11 0 10.221 4.721 5.5 0 .779.779 0 5.5 4.721 10.221 0 11 .779 6.279 5.5z"
          />
        </svg>
      </div>
    </div>

    <!-- Login Page -->
    <loginPage v-if="!loggedIn" @login="loggedIn = true" />

    <!-- Main Content -->
    <div v-if="loggedIn">
      <!-- Sidebar -->
      <aside :class="['sidebar', { 'sidebar--extended': isSidebarExtended }]" @mouseover="extendSidebar"
        @mouseleave="collapseSidebar">
        <nav>
          <ul class="sidebar-top">
            <li @click="setActiveComponent('personaList')" :class="{ active: activeComponent === 'personaList' }">
              <img src="/jester-white.svg" class="jester-logo">
              <span class="label" :class="{ 'label--visible': isSidebarExtended }">Hub</span>
            </li>
            <li @click="setActiveComponent('createPersona')" :class="{ active: activeComponent === 'createPersona' }">
              <i class="fa-solid fa-plus"></i>
              <span class="label" :class="{ 'label--visible': isSidebarExtended }">Create Persona</span>
            </li>
            <li @click="setActiveComponent('marketplace')" :class="{ active: activeComponent === 'marketplace' }">
              <i class="fa-solid fa-user"></i>
              <span class="label" :class="{ 'label--visible': isSidebarExtended }">Profile</span>
            </li>
            <li @click="setActiveComponent('fakeMic')" :class="{ active: activeComponent === 'fakeMic' }">
              <i class="fa-solid fa-microphone"></i>
              <span class="label" :class="{ 'label--visible': isSidebarExtended }">Fake Mic</span>
            </li>
            <li @click="setActiveComponent('appProfile')" :class="{ active: activeComponent === 'appProfile' }">
              <i class="fa-solid fa-user"></i>
              <span class="label" :class="{ 'label--visible': isSidebarExtended }">Profile</span>
            </li>
          </ul>
          <ul class="sidebar-bottom">
            <li @click="setActiveComponent('appSettings')" :class="{ active: activeComponent === 'appSettings' }">
              <i class="fa-solid fa-gear"></i>
              <span class="label" :class="{ 'label--visible': isSidebarExtended }">Settings</span>
            </li>
          </ul>
        </nav>
      </aside>

      <!-- Main Content Area -->
      <main class="main-content">
        <component :is="activeComponent" />
      </main>
    </div>
  </div>
</template>

<script>
import personaList from "/src/components/personaList.vue";
import appProfile from "/src/components/appProfile.vue";
import appSettings from "/src/components/appSettings.vue";
import fakeMic from "/src/components/fakeMic.vue";
import createPersona from "/src/components/createPersona.vue";
import loginPage from "./components/loginPage.vue";
import marketplace from "/src/components/marketplace.vue"
import { getCurrentWindow } from '@tauri-apps/api/window';
import "@fortawesome/fontawesome-free/css/all.css";

export default {
  data() {
    return {
      isSidebarExtended: false,
      loggedIn: false,
      activeComponent: "personaList", // Default component
      window: getCurrentWindow()
    };
  },
  components: {
    personaList,
    appProfile,
    appSettings,
    fakeMic,
    createPersona,
    loginPage,
    marketplace
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
    async closeWindow() {
      try {
        // Send POST to localhost:3000/killme, await the response, and print the output to console
        this.window.hide();

        const response = await fetch("http://localhost:3000/killme", {
          method: "POST"
        });
        const data = await response.text();
        console.log(data);
      } catch (error) {
        console.error(error);
      }
      this.window.close();
    }
  },
};
</script>

<style scoped>
/* App Container */
.app-container {
  display: flex;
  min-height: 100vh;
  min-width: 100vw;
  background:
    /* Overarching Gradient */
    radial-gradient(circle at bottom right,
      rgb(75, 80, 90),
      rgba(30, 34, 40, 0.7) 70%,
      rgba(20, 23, 28, 0) 100%),
    /* Polygonal and Diagonal Shapes */
    linear-gradient(150deg,
      rgba(65, 70, 78, 0.5) 30%,
      transparent 30%),
    linear-gradient(40deg,
      rgba(55, 60, 67, 0.4) 30%,
      transparent 30%),
    linear-gradient(210deg,
      rgba(42, 46, 52, 0.5) 30%,
      transparent 30%),
    linear-gradient(330deg,
      rgba(35, 40, 45, 0.4) 30%,
      transparent 30%),
    /* Additional Angular Shapes for More Variation */
    linear-gradient(120deg,
      rgba(48, 53, 60, 0.3) 40%,
      transparent 40%),
    linear-gradient(290deg,
      rgba(28, 33, 37, 0.3) 40%,
      transparent 40%),
    linear-gradient(260deg,
      rgba(38, 43, 48, 0.35) 50%,
      transparent 50%);
  background-size: cover;
  background-blend-mode: overlay;
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px) saturate(100%);
  /* WebKit fallback for Safari */
}

/* Jester Logo */
.jester-logo {
  width: 30px;
  height: 30px;
}

/* Sidebar */
.sidebar {
  width: 60px;
  transition: width 0.3s ease;
  height: 100vh;
  display: flex;
  flex-direction: column;
  background: linear-gradient(145deg, rgba(35, 37, 42, 0.7), rgba(40, 43, 48, 0.5));
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px) saturate(100%);
  /* WebKit fallback for Safari */
  border-right: 1px solid rgba(255, 255, 255, 0.5);
  overflow: hidden;
  position: fixed;
  top: 0;
  left: 0;
  z-index: 1;
}

.sidebar--extended {
  width: 200px;
}

/* Sidebar Navigation */
.sidebar nav {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.sidebar nav ul {
  list-style: none;
  padding: 0;
  margin: 0;
}

.sidebar-top {
  flex-grow: 1;
}

.sidebar-bottom {
  /* Align to bottom */
  margin-bottom: 20px;
}

/* Sidebar Items */
.sidebar nav ul li {
  padding: 15px;
  cursor: pointer;
  color: white;
  transition: background 0.3s ease;
  display: flex;
  align-items: center;
  position: relative;
}

.fa-solid {
  font-size: 30px;
}

.sidebar nav ul li:hover {
  background: rgba(255, 255, 255, 0.1);
}

.sidebar nav ul li.active::before {
  content: '';
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 4px;
  background: #ff3b3b;
}

.sidebar nav ul li.active {
  background: rgba(255, 255, 255, 0.1);
}

/* Labels */
.label {
  display: inline-block;
  opacity: 0;
  transform: translateX(-20px);
  transition: opacity 0.3s ease, transform 0.3s ease;
  white-space: nowrap;
  margin-left: 10px;
}

.label--visible {
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
  margin-left: 60px;
  /* Adjusted for fixed sidebar */
  transition: margin-left 0.3s ease; /* Smooth transition */
}

.sidebar--extended~.main-content {
  margin-left: 200px;
  /* Adjust when sidebar is extended */
}
</style>

<style>
/* Titlebar */
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
  width: 60px;
  height: 30px;
  user-select: none;
  -webkit-user-select: none;
}

.titlebar-button:hover {
  background: rgba(87, 39, 39, 0.5);
}

.titlebar .fa-solid {
  color: #d1d1d1;
  font-size: 20px;
}

.titlebar svg {
  width: 13px;
  height: 13px;
}
</style>
