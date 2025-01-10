<template>
  <div class="profile-container">
    <h2 class="unselectable">My Personas</h2>
    <div class="persona-container">
    <div v-for="persona in personas" :key="persona.id"  :class="['persona-box', { selected: persona === selectedPersona }]">
      <img
        :src="persona.photo"
        alt="Persona Photo"
        class="persona-photo"
        @error="setDefaultPhoto(persona)"
      />
      <div class="persona-info">
        <h3 class="persona-name unselectable">{{ persona.name }}</h3>
        <p v-if="showDescription" class="persona-desc unselectable"   :style="{ maxHeight: this.windowHeight >= 900 ? '100px' : '60px' }">{{ persona.description }}</p>
      </div>
      <button v-if="persona.sold==false"  class="select-button" @click="sellPersona(persona)">
        Sell
      </button>
      <button v-if="persona.sold==true"  class="select-button" @click="removePersona(persona)">
        Remove
      </button>
    </div>
  </div>
  </div>
</template>
<script>
import personaDetail from "/src/components/personaDetail.vue";
import { globalState } from "/src/eventBus.js";
export default {
  components: {
    personaDetail,
  },
  name: "appProfile",
  data() {
    return {
      iconSrc: "../../public/icon-128.png",
      personas: null,
      windowWidth: window.innerWidth,
      windowHeight: window.innerHeight,
      selectedPersona: globalState.selectedPersona,
      pageUrl: "/api",
    };
  },
  computed: {
    showDescription() {
      return this.windowWidth >= 600 && this.windowHeight >=741; // Set width threshold for showing description
    },
  },
  methods: {
    async getUserSellablePersonas(username) {
      try {
        const response = await fetch(this.pageUrl + "/personas", {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            username: username,
          }),
        });

        if (!response.ok) {
          throw new Error("Failed to fetch personas");
        }

        const data = await response.json();

      const userPersonas = data.map(persona => ({
      ...persona,
      photo: `data:image/jpeg;base64,${persona.imageBase64}`, // Assuming PNG format
      }
      ));

      try {
        const response2 = await fetch(this.pageUrl + "/marketplace/seller-personas", {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            username: username,
          }),
        });
        if (!response2.ok) {
          throw new Error("Failed to fetch personas");
        }
        const data2 = await response2.json();
        const sellerPersonas = data2.map(item => ({
          ...item.persona,
          photo: `data:image/jpeg;base64,${item.persona.imageBase64}`, // Assuming PNG format
          }
        ));
        this.personas = userPersonas
          .filter(p => !sellerPersonas.some(sp => sp.name === p.name))
          .map(p => ({ ...p, sold: false, owner: globalState.username === p.ownerName }));
        const soldPersonas = sellerPersonas.map(sp => ({ ...sp, sold: true, owner: globalState.username === sp.owner }));
        this.personas = this.personas.concat(soldPersonas);
      } catch (error) {
        console.error("Failed to fetch personas");  
        console.log(error);
      }
      } catch (error) {
        console.error("Error fetching personas:", error);
      }
    },
    sellPersona(persona) {
      const sendRequest = async () => {
        const url = this.pageUrl + "/marketplace/sell-persona"; 
        const response = await fetch(url, {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            username: globalState.username,
            personaName: persona.name,
          }),
        });
      if (!response.ok) {
        console.error("Failed to sell persona");
      } else {
        persona.sold = true;
        alert("Persona sold successfully!");
      }
    }
    sendRequest();
  },
  removePersona(persona) {
      const sendRequest = async () => {
        const url = this.pageUrl + "/marketplace/remove"; 
        const response = await fetch(url, {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            username: globalState.username,
            personaName: persona.name,
          }),
        });
      if (!response.ok) {
        console.error("Failed to remove persona");
      } else {
        persona.sold = false;
        alert("Persona removed successfully!");
      }
    }
    sendRequest();
  },
    setDefaultPhoto(persona) {
      persona.photo = "profilePhotos/default.webp";
    },
    updateWindowSize() {
      this.windowWidth = window.innerWidth;
      this.windowHeight = window.innerHeight;
    },  

  },
  mounted() {
    this.getUserSellablePersonas(globalState.username);
    window.addEventListener("resize", this.updateWindowSize);
  },
  beforeDestroy() {
    window.removeEventListener("resize", this.updateWindowSize);
  },
};

</script>


<style scoped>
.profile-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  width: 100%;
  background: linear-gradient(145deg, rgba(35, 37, 42, 0.2), rgba(40, 43, 48, 0.2));
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px) saturate(100%);
  padding: 2.5rem;
  border-radius: 12px;
  box-shadow: 0 8px 16px rgba(0, 0, 0, 0.3);
  border: 1px solid rgba(255, 255, 255, 0.2);
  margin: 30px;
}


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
  height: 100px; 
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

input {
  margin-right: 10px;
}
.persona-container {
  display: flex;
  height: fit-content;
  padding-bottom: 10vh;
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
  min-width: 200px;
  height: 36vh;
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


.persona-box.selected {
  transform: scale(1.1);
  box-shadow: 0 4px 26px #ff2525;
  border: 2px solid #ff2525;
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

.persona-name {
  font-size: 1.5em;
  color: #d1d1d1;
  word-wrap: break-word;
}


.persona-desc {
  color: #d1d1d1;
  max-height: 60px; /* Limit the height */
  overflow-y: scroll;
  text-overflow: ellipsis;
  white-space: normal; /* Prevent breaking words */
  word-wrap: break-word;
  margin-bottom: 10px; /* Ensure spacing before button */
}
/* Custom modern and minimal scrollbar styling for persona description */
.persona-desc::-webkit-scrollbar {
    width: 6px; /* Thin scrollbar */
}

.persona-desc::-webkit-scrollbar-track {
    background: transparent; /* Invisible track for a minimal look */
}

.persona-desc::-webkit-scrollbar-thumb {
    background: #474a50; /* Matches your button color */
    border-radius: 10px;
    border: 2px solid transparent;
    background-clip: padding-box;
    transition: background-color 0.3s ease;
}

.persona-desc::-webkit-scrollbar-thumb:hover {
    background: #24262b; /* Slightly brighter on hover for interaction feedback */
}


.select-button, 
.selected-button {
  align-self: center; /* Keep the button centered at the bottom */
  margin-top: auto;  /* Push the button to the bottom of the flex container */
}
.selected-button {
  background-color: #ab162b;
  color: white;
  border: none;
  border-radius: 5px;
  padding: 5px 10px;
  cursor: pointer;
  box-shadow: 0 10px 15px rgba(0, 0, 0, 0.5) ;
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
