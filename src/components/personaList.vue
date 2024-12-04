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
        <p v-if="showDescription" class="persona-desc unselectable">{{ persona.description }}</p>
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
import personasData from "/src/assets/personas.json";
import personaDetail from "/src/components/personaDetail.vue";
import { globalState } from "/src/eventBus.js";

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
      windowWidth: window.innerWidth,
      pageUrl: "http://localhost:3000",
    };
  },
  computed: {
    showDescription() {
      return this.windowWidth >= 900; // Set width threshold for showing description
    },
  },
  methods: {
    async getUserPersonas(username) {
      try {
        const response = await fetch(this.pageUrl + "/getUserPersonas", {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({ username: username }),
        });

        if (!response.ok) {
          throw new Error("Failed to fetch personas");
        }

        const data = await response.json();
        this.personas = data.personas;
      } catch (error) {
        console.error("Error fetching personas:", error);
      }
    },
    selectPersona(persona) {
      const sendRequest = async () => {
        const url = this.pageUrl + "/browse"; // Replace PORT with your server's port
        const response = await fetch(url, {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            urls: persona.urls,
          }),
        });
        console.log(response);
        console.log("Selected persona:", persona);
        console.log("links:", persona.urls); 
    }
    sendRequest();
  },
    setDefaultPhoto(persona) {
      persona.photo = "profilePhotos/default.webp";
    },
    updateWindowWidth() {
      this.windowWidth = window.innerWidth;
    },

  },
  mounted() {
    this.getUserPersonas(globalState.username);
    console.log("User:", globalState.username);
    window.addEventListener("resize", this.updateWindowWidth);
  },
  beforeDestroy() {
    window.removeEventListener("resize", this.updateWindowWidth);
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
  height: 90vh;
  flex-wrap: wrap; /* Allows wrapping to new rows */
  justify-content: center;
  gap: 30px; /* Adjusts spacing between items */
  align-items: center;
  margin-top: 20px;
  width: 100%;
  overflow-y: auto;
  overflow-x: hidden;
}

.persona-box {
  background: linear-gradient(145deg, rgba(35, 37, 42, 0.7), rgba(40, 43, 48, 0.5));
  height: 30vh;
  border-radius: 20px;
  padding: 1.5em;
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  box-shadow: 0 8px 16px rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px) saturate(100%); /* WebKit fallback for Safari */
  transition: transform 0.2s;
}

@media (min-width: 800px) {
  .persona-box {
    width: calc(25% - 10px); 
  }
}

@media (min-width: 1000px) {
  .persona-box {
    width: calc(15% - 10px); /* 4 per row for even larger screens */
  }
}
/* Custom scrollbar styling */
.persona-container::-webkit-scrollbar {
  width: 8px;
}

.persona-container::-webkit-scrollbar-track {
  background: transparent;
  box-shadow: inset 0 0 6px rgba(255, 255, 255, 0.3);
  border-radius: 10px;
}

.persona-container::-webkit-scrollbar-thumb {
  background-color: #ab162b ;
  border-radius: 10px;
  border: transparent;
}

.persona-photo {
  width: 70px;
  height: 70px;
  border-radius: 50%;
  margin-right: 10px;
  box-shadow: 0 10px 15px rgba(0, 0, 0, 0.5) ;

}

.persona-info {
  width: 100%;
  flex: 1;
}

.persona-name {
  font-size: 1.5em;
  color: #d1d1d1;
}
.persona-desc{
  color: #d1d1d1;
  height: 2vh;
    
}

.select-button {
  background-color: #ab162b;
  color: white;
  border: none;
  border-radius: 5px;
  padding: 5px 10px;
  cursor: pointer;
  box-shadow: 0 10px 15px rgba(0, 0, 0, 0.5) ;
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
.unselectable {
  -webkit-touch-callout: none;
  -webkit-user-select: none;
  -khtml-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
}
</style>
