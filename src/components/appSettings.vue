<template>

  <div class="settings-container">
    <div class="slidecontainer">
      <input v-model="concurrentPages" type="range" min="1" max="20" class="slider">
      <p>Concurrent Pages: {{ concurrentPages }}</p>
    </div>
    <div class="slidecontainer">
      <input v-model="pageTime" type="range" min="1" max="300" class="slider">
      <p>Page Time: {{ pageTime }}</p>
    </div>
    <p v-if="message" class="message">{{ message }}</p>
    <p>Headless Mode</p>
    <input type="checkbox" v-model="isHeadless" />
    <p>Loop Links</p>
    <input type="checkbox" v-model="loopLinks" />
    <button @click="logout">Logout
    </button>
  </div>
</template>

<script>
export default {
  name: "appSettings",
  data() {
    return {
      apiKey: "", // Stores the API key
      isHeadless: false,
      message: "", // For user feedback
      concurrentPages: 3,
      pageTime: 3,
      loopLinks: false,
    };
  },
  methods: {
    storeApiKey() {
      try {
        // Store the API key in localStorage
        this.storeUserSetting("userid", "apiKey", this.apiKey);

        this.message = "API key saved!";
      } catch (error) {
        console.error("Failed to save API key:", error);
        this.message = "Failed to save API key.";
      }

      // Clear the message after 2 seconds
      setTimeout(() => {
        this.message = "";
      }, 2000);
    },
    logout() {
      this.$emit("logout");
    },
    loadSettings(){
      const sendRequest = async () => {
        const url = "http://localhost:3000" + "/config";
        const response = await fetch(url, {
          method: "GET",
          headers: {
            "Content-Type": "application/json",
          },
        }).then((response) => response.json())
          .then((data) => {
            this.concurrentPages = data.concurrent_pages;
            this.pageTime = data.page_time / 1000;
            this.isHeadless = data.headless;
            this.loopLinks = data.loop_links;
          });
      }
      sendRequest();
    },
    storeSettings() {
      const sendRequest = async () => {
        const url = "http://localhost:3000" + "/config";
        const response = await fetch(url, {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            concurrent_pages: this.concurrentPages * 1,
            page_time: this.pageTime * 1000,
            headless: this.isHeadless,
            loop_links: this.loopLinks,
          }),
        })
      }
      sendRequest();
    },

  },
  mounted() {
    // Load the API key when the component is mounted
    this.loadSettings();
  },
  watch: {
    concurrentPages(newValue) {
      this.storeSettings();
    },
    pageTime(newValue) {
      this.storeSettings();
    },
    isHeadless(newValue) {
      this.storeSettings();
    },
    loopLinks(newValue) {
      this.storeSettings();
    },
  },
};
</script>

<style scoped>
button {
  width: 100%;
  padding: 1rem;
  background: linear-gradient(45deg, #ff3b3b, #ff5c5c);
  color: white;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  font-size: 1.1rem;
  font-weight: 600;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  transition: all 0.3s ease;
}

button:hover {
  background: linear-gradient(45deg, #ff2525, #ff4747);
  transform: translateY(-1px);
}

button:active {
  transform: translateY(0);
}

.slider {
  --webkit-appearance: none;
  width: 90%;
  height: 15px;
  border-radius: 5px;  
  background: #d3d3d3;
  outline: none;
  opacity: 0.7;
  --webkit-transition: .2s;
  transition: opacity .2s;
}

.slider::-webkit-slider-thumb {
  --webkit-appearance: none;
  appearance: none;
  width: 25px;
  height: 25px;
  border-radius: 50%; 
  background: #04AA6D;
  cursor: pointer;
}

.slider::-moz-range-thumb {
  width: 25px;
  height: 25px;
  border-radius: 50%;
  background: #04AA6D;
  cursor: pointer;
} 

.settings-container {
  margin: 20px;
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 20px;
  background: linear-gradient(145deg, rgba(35, 37, 42, 0.7), rgba(40, 43, 48, 0.5));
  border-radius: 10px;
  color: white;
  font-family: Arial, sans-serif;
}

input {
  padding: 10px;
  border: none;
  border-radius: 5px;
  background: rgba(255, 255, 255, 0.2);
  color: white;
}

input:focus {
  outline: 2px solid #ff3b3b;
}

.message {
  font-size: 0.9rem;
  color: #28a745;
}
</style>
