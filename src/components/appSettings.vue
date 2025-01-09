<template>

  <div class="settings-container">
    <button @click="logout">Logout
    </button>
    <div class="slidecontainer">
      <input v-model="concurrentPages" type="range" min="1" max="20" value="3" class="slider" id="myRange">
      <p>Concurrent Pages: {{ concurrentPages }}</p>
    </div>
    <div class="slidecontainer">
      <input v-model="pageTime" type="range" min="1" max="300" value="3" class="slider" id="myRange">
      <p>Page Time: {{ pageTime }}</p>
    </div>
    <p v-if="message" class="message">{{ message }}</p>
    <p>Headless Mode</p>
    <input type="checkbox" v-model="isHeadless" @change="storeIsHeadless('userid', isHeadless)" />
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
    loadApiKey() {
      try {
        // Load the API key from localStorage
        const savedApiKey = this.getUserSetting("userid", "apiKey");

        // Set the API key if it exists
        if (savedApiKey) {
          this.apiKey = savedApiKey;
        }
      } catch (error) {
        console.warn("No API key found:", error);
      }
    },
    storeIsHeadless(userId, isHeadless) {
      // Store the headless mode setting
      this.storeUserSetting(userId, "isHeadless", isHeadless);
    },
    getIsHeadless(userId) {
      // Retrieve the headless mode setting
      return this.getUserSetting(userId, "isHeadless");
    },
    loadIsHeadless() {
      // Load the headless mode setting
      this.isHeadless = this.getIsHeadless("userid");
    },
    storeUserSetting(userId, key, value) {
      // Compose a unique key for the user and setting
      const settingKey = `${userId}-${key}`;
      localStorage.setItem(settingKey, JSON.stringify(value));
    },

    getUserSetting(userId, key) {
      // Retrieve the setting for the specific user
      const settingKey = `${userId}-${key}`;
      const value = localStorage.getItem(settingKey);
      return value ? JSON.parse(value) : null;
    },
    logout() {
      this.$emit("logout");
    },
  },
  mounted() {
    // Load the API key when the component is mounted
    this.loadApiKey();
    this.loadIsHeadless();
  },
  watch: {
    concurrentPages(newValue) {
    },
    pageTime(newValue) {
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
