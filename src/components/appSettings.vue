<template>
  <div class="settings-container">
    <p>API Key</p>
    <input
      v-model="apiKey"
      @input="storeApiKey"
      placeholder="Enter your API key"
    />
    <p v-if="message" class="message">{{ message }}</p>
    <p>Headless Mode</p>
    <input
      type="checkbox"
      v-model="isHeadless"
      @change="storeIsHeadless('userid', isHeadless)"
    />
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
      this.isHeadless = this.getIsHeadless("userid");},
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
  },
  mounted() {
    // Load the API key when the component is mounted
    this.loadApiKey();
    this.loadIsHeadless();
  },
};
</script>

<style scoped>
.settings-container {
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 20px;
  background: rgba(255, 255, 255, 0.1);
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
