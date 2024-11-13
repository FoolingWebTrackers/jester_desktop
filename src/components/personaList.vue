<template>
  <!-- Persona Container -->
  <div class="persona-container">
    <div v-for="persona in personas" :key="persona.id" class="persona-box">
      <img
        :src="persona.photo"
        alt="Persona Photo"
        class="persona-photo"
        @error="setDefaultPhoto(persona)"
      />
      <div class="persona-info">
        <h3 class="persona-name unselectable">{{ persona.name }}</h3>
      </div>
      <button class="select-button" @click="selectPersona(persona)">
        Select
      </button>
      <Transition mode="out-in">
        <personaDetail
          v-if="selectedPersona"
          :persona="selectedPersona"
          @close="selectedPersona = null"
        />
      </Transition>
    </div>
  </div>
</template>

<script>
/* global chrome */
import personasData from "/src/assets/personas.json";
import personaDetail from "/src/components/personaDetail.vue";
export default {
  components: {
    personaDetail,
  },
  name: "personaList",
  data() {
    return {
      iconSrc: "../../public/icon-128.png",
      selectedPersona: null,
      personas: personasData.map((persona) => ({
        ...persona,
        photo: persona.photo,
      })),
    };
  },
  methods: {
    selectPersona(persona) {
      // Implement logic for selecting a persona
      this.selectedPersona = persona;
    },
    closeTabs() {
      for (const tabId of this.tabIds) {
        chrome.tabs.remove(tabId);
      }
    },
    setDefaultPhoto(persona) {
      persona.photo = "profilePhotos/default.webp";
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
  margin-top: 5px; /* Space between button and label */
  color: #d1d1d1; /* Adjust label color as needed */
  -webkit-touch-callout: none;
  -webkit-user-select: none;
  -khtml-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
}

input {
  margin-right: 10px;
}
.persona-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  margin-top: 20px;
  width: 100%;
  max-height: 397px; /* Adjust the height to fit within main container */
  overflow-y: auto;
  overflow-x: hidden;
}

/* Custom scrollbar styling */
.persona-container::-webkit-scrollbar {
  width: 8px;
}

.persona-container::-webkit-scrollbar-track {
  background: #2e2e2e;
  border-radius: 10px;
}

.persona-container::-webkit-scrollbar-thumb {
  background-color: #4a0000;
  border-radius: 10px;
  border: 2px solid #2e2e2e;
}

.persona-box {
  height: fit-content;
  display: flex;
  align-items: center;
  background-color: #2e2e2e;
  border: 1px solid #383636;
  border-radius: 5px;
  padding: 10px;
  margin: 2px;
  width: 90%;
  min-height: 63px;
}

.persona-photo {
  width: 50px;
  height: 50px;
  border-radius: 50%;
  margin-right: 10px;
}

.persona-info {
  flex: 1;
}

.persona-name {
  font-size: 16px;
  color: #d1d1d1;
}

.select-button {
  background-color: #4a0000;
  color: white;
  border: none;
  border-radius: 5px;
  padding: 5px 10px;
  cursor: pointer;
}
.fa-solid {
  font-size: 20px;
  position: relative;
  color: #d1d1d1;
}
.unselectable {
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
</style>
