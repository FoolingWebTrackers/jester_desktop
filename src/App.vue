<template>
  <div class="mainPage-header">
    <img :src="'icon-128.png'" alt="Icon" class="icon" />
  </div>

  <div class="mainPage-container">
    <div class="button-container">
      <div class="button-group">
        <button class="icon-button" @click="openCreatePersona">
          <i class="fa-solid fa-plus"></i>
        </button>
        <label>Create</label>
        <label style="margin-top: -3px">Personas</label>
      </div>
      <div class="button-group">
        <button class="icon-button" @click="openFakeMic">
          <i class="fa-solid fa-microphone"></i>
        </button>
        <label>Fake Mic</label>
      </div>
      <div class="button-group">
        <button class="icon-button" @click="openProfile">
          <i class="fa-solid fa-user"></i>
        </button>
        <label>Profile</label>
      </div>
      <Transition mode="out-in">
        <div
          class="button-group"
          @click="openSettings"
          v-if="currentView === 'mainPage'"
        >
          <button class="icon-button">
            <i class="fa-solid fa-gear"></i>
          </button>
          <label>Settings</label>
        </div>
        <div class="button-group" @click="openMainPage" v-else>
          <button class="icon-button">
            <i class="fa-solid fa-circle-left"></i>
          </button>
          <label>Back</label>
        </div>
      </Transition>
    </div>
    <Transition mode="out-in">
      <component :is="currentView" />
    </Transition>
  </div>
  <div class="mainPage-footer unselectable">Jester</div>
</template>

<script>
import personasData from "@/assets/personas.json";
import mainPage from "@/components/personaList.vue";
import createPersona from "@/components/createPersona.vue";
import fakeMic from "@/components/fakeMic.vue";
import appProfile from "@/components/appProfile.vue";
import settings from "@/components/appSettings.vue";
import '@fortawesome/fontawesome-free/css/all.css';

export default {
  components: {
    mainPage,
    createPersona,
    fakeMic,
    appProfile,
    settings,
  },
  data() {
    return {
      currentView: "mainPage",
      iconSrc: "../../public/icon-128.png",
      tabIds: [],
      personas: personasData.map((persona) => ({
        ...persona,
        photo: persona.photo,
      })),
    };
  },
  methods: {
    openMainPage() {
      this.currentView = "mainPage";
    },
    openCreatePersona() {
      this.currentView = "createPersona";
    },
    openFakeMic() {
      this.currentView = "fakeMic";
    },
    openProfile() {
      this.currentView = "appProfile";
    },
    openSettings() {
      this.currentView = "settings";
    },
    backButton() {
      this.currentView = "mainPage";
    },
  },
};
</script>

<style scoped>
.v-move,
.v-enter-active,
.v-leave-active {
  transition: 0.3s ease;
}

.v-enter-from,
.v-leave-to {
  opacity: 0;
  transform: translateY(10px);
}

.icon {
  width: 45px; /* Adjust the size as needed */
  height: auto; /* Maintain aspect ratio */
}
.icon-button {
  width: 50px;
  height: 50px;
  background-color: #4a0000;
  border-radius: 50%;
  border: none;
  cursor: pointer;
}
.button-container {
  align-items: baseline;
  display: flex;
  width: 100%;
  justify-content: space-evenly;
  height: 100px; /* Adjusted height to accommodate labels */
}
.button-group {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  height: 100%;
}

label {
  margin-top: 5px;
  color: #d1d1d1;
  -webkit-touch-callout: none;
  -webkit-user-select: none;
  -khtml-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
}

.mainPage-footer {
  position: relative;
  display: flex;
  justify-content: center;
  align-items: center;
  height: 30px;
  background: #383636;
  box-shadow: 0px -10px 20px rgba(0, 0, 0, 0.5);
  color: #761626;
  font-size: 15pt;
}
.mainPage-header {
  position: relative;
  display: flex;
  justify-content: center;
  align-items: center;
  background: #383636;
  height: 70px;
  box-shadow: 0px 10px 20px rgba(0, 0, 0, 0.5);
}
.mainPage-container {
  display: flow-root;
  width: 300px;
  height: 500px;
  background: #1e1e1e;
  margin: 0 px;
  padding: 0px;
}
.icon {
  width: 45px; /* Adjust the size as needed */
  height: auto; /* Maintain aspect ratio */
}
.icon-button {
  width: 50px;
  height: 50px;
  background-color: #4a0000;
  border-radius: 50%;
  border: none;
  cursor: pointer;
}
.button-container {
  align-items: baseline;
  display: flex;
  width: 100%;
  justify-content: space-evenly;
  height: 100px; /* Adjusted height to accommodate labels */
}
.button-group {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  height: 100%;
}

label {
  margin-top: 5px; /* Space between button and label */
  color: #d1d1d1; /* Adjust label color as needed */
  -webkit-touch-callout: none;
  -webkit-user-select: none;
  -khtml-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
}
</style>
<style>
body {
  overflow: hidden; /* Hide scrollbars */
  margin: 0px;
  width: 300px;
  height: 600px;
  background: transparent;
}
.unselectable {
  -webkit-touch-callout: none;
  -webkit-user-select: none;
  -khtml-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
}
.fa-solid {
  font-size: 20px;
  position: relative;
  color: #d1d1d1;
}
</style>
