<template>
  <div class="create-persona-page">
    <div v-if="loading" class="loading-screen">
      <span class="loader"></span>      
      <p class="loading-text">Loading...</p>
    </div>
    <div v-else class="content-wrapper">
      <!-- Left Section -->
      <div class="info-section">
        <h1>Create Your Persona</h1>
        <p class="description">
          Design a unique browsing profile that will help generate automated traffic to confuse web trackers. 
          The more detailed your description, the better our AI can generate relevant browsing patterns.
        </p>
        <div class="tips-container">
          <h3>Tips for Effective Personas</h3>
          <ul>
            <li>
              <i class="fa-solid fa-star"></i>
              Define clear interests and hobbies
            </li>
            <li>
              <i class="fa-solid fa-star"></i>
              Include demographic details
            </li>
            <li>
              <i class="fa-solid fa-star"></i>
              Specify preferred websites or content
            </li>
          </ul>
        </div>
      </div>

      <!-- Right Section -->
      <div class="form-section">
        <div class="form-container">
          <form @submit.prevent="createPersona">
            <div class="form-group">
              <label for="persona-name">Persona Name</label>
              <input
                id="persona-name"
                v-model="personaName"
                type="text"
                required
                placeholder="Enter a memorable name"
              />
            </div>
            
            <div class="form-group">
              <label for="persona-desc">Persona Description</label>
              <textarea
                id="persona-desc"
                v-model="personaDesc"
                required
                rows="8"
                placeholder="Describe your persona's interests, demographics, and preferred websites. For example: 'A cryptocurrency enthusiast who frequently visits trading platforms, blockchain news sites, and NFT marketplaces...'"
              ></textarea>
            </div>

            <div class="options-group">
              <div class="checkbox-container">
                <input type="checkbox" id="generate-photo" v-model="generatePhoto" />
                <label for="generate-photo">
                  <i class="fa-solid fa-image"></i>
                  Generate AI Photo
                </label>
              </div>
            </div>

            <button type="submit">
              <i class="fa-solid fa-wand-magic-sparkles"></i>
              Create Persona
            </button>
          </form>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { globalState } from "/src/eventBus.js";
export default {
  name: "createPersona",
  data() {
    return {
      personaName: "",
      username: globalState.username,
      personaDesc: "",
      pageUrl: "/api",
      generatePhoto: false,
      loading: false
    };
  },
  methods: {
    async createPersona() {
      const { username, personaName, personaDesc } = this;
      if (!username || !personaName || !personaDesc) {
        alert("All fields are required");
        return;
      }


      const userData = {
            username: username, 
            personaName: personaName, 
            description: personaDesc,
            generatePhoto: this.generatePhoto
        };
        this.loading=true
      try {
        const url = this.pageUrl + "/personas/register";
        const response = await fetch(url, {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify(userData),
        });

        if (!response.ok) {
          throw new Error("Failed to create persona");
        }

        const data = await response.json();
        alert("Persona created successfully");
        console.log(data);
      } catch (error) {
        console.error("Error creating persona:", error);
        alert("Error creating persona");
      }
      finally{
        this.loading=false
      }
    },

  },
};
</script>

<style scoped>
.create-persona-page {
  position: relative;
  min-height: 100vh;
  width: 100%;
  padding: 2rem;
  box-sizing: border-box;
}
.loading-screen {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: rgba(0, 0, 0, 0.8);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}
.loading-text {
  color: #fff;
  font-size: 1.5rem;
  margin-top: 1rem;
}

.loader {
        width: 84px;
        height: 84px;
        position: relative;
        overflow: hidden;
      }
      .loader:before , .loader:after {
        content: "";
        position: absolute;
        left: 50%;
        bottom: 0;
        width:64px;
        height: 64px;
        border-radius: 50%;
        background:#FFF;
        transform: translate(-50% , 100%)  scale(0);
        animation: push 2s infinite ease-in;
      }
      .loader:after {
      animation-delay: 1s;
      }
      @keyframes push {
          0% {
            transform: translate(-50% , 100%)  scale(1);
          }
          15% , 25%{
            transform: translate(-50% , 50%)  scale(1);
          }
        50% , 75% {
            transform: translate(-50%, -30%) scale(0.5);
          }
        80%,  100% {
            transform: translate(-50%, -50%) scale(0);
          }
      }

.content-wrapper {
  display: grid;
  grid-template-columns: 300px minmax(400px, 5fr);
  gap: 2rem;
  margin: 0 auto;
  width: 100%;
  max-width: 3000px;
}

/* Left Section Styles */
.info-section {
  position: sticky;
  top: 2rem;
  width: 300px;
}

.info-section h1 {
  font-size: 2.5rem;
  margin-bottom: 1.5rem;
  background: linear-gradient(45deg, #ff3b3b, #ff8080);
  -webkit-background-clip: text;
  background-clip: text;
  color: transparent;
  font-weight: 700;
}

.description {
  font-size: 1.1rem;
  line-height: 1.6;
  color: #e1e1e1;
  margin-bottom: 2rem;
}

.tips-container {
  background: linear-gradient(145deg, rgba(35, 37, 42, 0.4), rgba(40, 43, 48, 0.2));
  backdrop-filter: blur(12px);
  border-radius: 12px;
  padding: 1.5rem;
  border: 1px solid rgba(255, 255, 255, 0.1);
}

.tips-container h3 {
  color: #f1f1f1;
  margin-bottom: 1rem;
  font-size: 1.2rem;
}

.tips-container ul {
  list-style: none;
  padding: 0;
}

.tips-container li {
  margin-bottom: 1rem;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  color: #e1e1e1;
}

.tips-container i {
  color: #ff3b3b;
  font-size: 0.8rem;
}

/* Right Section Styles */
.form-section {
  position: relative;
  width: 100%;
  max-width: 2500px;
}

.form-container {
  background: linear-gradient(145deg, rgba(35, 37, 42, 0.7), rgba(40, 43, 48, 0.5));
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px) saturate(100%);
  padding: 2.5rem;
  border-radius: 12px;
  box-shadow: 0 8px 16px rgba(0, 0, 0, 0.3);
  border: 1px solid rgba(255, 255, 255, 0.2);
  width: 100%;
  box-sizing: border-box;
}

.form-group {
  margin-bottom: 1.5rem;
  width: 100%;
  box-sizing: border-box;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  color: #f1f1f1;
  font-weight: 500;
}

.form-group input,
.form-group textarea {
  width: 100%;
  padding: 1rem;
  background-color: rgba(42, 44, 49, 0.8);
  border: 1px solid #444;
  border-radius: 8px;
  color: #fff;
  box-sizing: border-box;
  font-size: 1rem;
}

.form-group input::placeholder,
.form-group textarea::placeholder {
  color: rgba(255, 255, 255, 0.4);
}

.form-group textarea {
  resize: vertical;
  min-height: 200px;
  line-height: 1.5;
}

.form-group input:focus,
.form-group textarea:focus {
  outline: none;
  border-color: #ff3b3b;
  box-shadow: 0 0 0 2px rgba(255, 59, 59, 0.2);
}

.options-group {
  margin-bottom: 2rem;
}

.checkbox-container {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.checkbox-container input[type="checkbox"] {
  width: 20px;
  height: 20px;
  cursor: pointer;
}

.checkbox-container label {
  color: #f1f1f1;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.checkbox-container i {
  color: #ff3b3b;
}

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

/* Hide info section when space is limited */
@media (max-width: 800px) {
  .content-wrapper {
    grid-template-columns: 1fr;
  }

  .info-section {
    display: none;
  }

  .form-section {
    max-width: none;
  }
}
</style>
