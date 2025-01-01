<template>
  <div class="login-page">
    <div class="form-container">
      <div v-if="showLoginForm">
        <h2>Login</h2>
        <form @submit.prevent="handleLogin">
          <div class="form-group">
            <label for="username">Username</label>
            <input type="text" id="username" v-model="loginUsername" required />
          </div>
          <div class="form-group">
            <label for="password">Password</label>
            <input
              :type="showPassword ? 'text' : 'password'"
              id="password"
              v-model="loginPassword"
              required
            />
          </div>
          <button type="submit">Login</button>
        </form>
        <p>
          Don't have an account?
          <a href="#" @click.prevent="switchToCreateUser">Create one</a>
        </p>
      </div>

      <div v-else>
        <h2>Create Account</h2>
        <form @submit.prevent="handleCreateUser">
          <div class="form-group">
            <label for="new-username">Username</label>
            <input
              type="text"
              id="new-username"
              v-model="newUsername"
              required
            />
          </div>
          <div class="form-group">
            <label for="new-password">Password</label>
            <input
              :type="showPassword ? 'text' : 'password'"
              id="new-password"
              v-model="newPassword"
              required
            />
          </div>
          <div class="form-group">
            <label for="confirm-password">Re-enter Password</label>
            <input
              :type="showPassword ? 'text' : 'password'"
              id="confirm-password"
              v-model="confirmPassword"
              required
            />
          </div>
          <div class="checkbox-container">
            <input type="checkbox" id="show-password" v-model="showPassword" />
            <label for="show-password">Show Password</label>
          </div>
          <button type="submit">Create Account</button>
        </form>
        <p>
          Already have an account?
          <a href="#" @click.prevent="switchToLogin">Login</a>
        </p>
      </div>
    </div>
    <div class="server-status" :class="{ online: isServerOnline }">
      Server Status: {{ isServerOnline ? 'Online' : 'Offline' }}
    </div>
  </div>
</template>

<script>
import { globalState } from "/src/eventBus.js";
export default {
  data() {
    return {
      showLoginForm: true, // Determines whether to show login or create user form
      showPassword: false, // Toggles password visibility
      loginUsername: "",
      loginPassword: "",
      newUsername: "",
      newPassword: "",
      confirmPassword: "",
      pageUrl: "/api",
      isServerOnline: false,
    };
  },
  methods: {
    toggleShowPassword() {
      this.showPassword = !this.showPassword;
    },
    switchToCreateUser() {
      this.showLoginForm = false;
    },
    switchToLogin() {
      this.showLoginForm = true;
    },
    generateSalt() {
      const array = new Uint8Array(256);
      crypto.getRandomValues(array);
      return Array.from(array)
        .map((byte) => byte.toString(16).padStart(2, "0"))
        .join("");
    },
    handleLogin() {
      // TODO: Implement login logic
      console.log("Logging in with:", this.loginUsername);
      this.authenticateUser(this.loginUsername, this.loginPassword);
      globalState.username = this.loginUsername;
    },
    handleCreateUser() {
      if (this.newPassword !== this.confirmPassword) {
        alert("Passwords do not match!");
        return;
      }
      // TODO: Implement user creation logic
      this.createUser(this.newUsername, this.newPassword);
    },
    authenticateUser(username, password) {
      const sendRequest = async () => {
        const url = this.pageUrl + "/users/authenticate";

        const userData = {
          username: username,
          password: password
        };

        try {
          const response = await fetch(url, {
            method: "POST",
            headers: {
              "Content-Type": "application/json",
            },
            body: JSON.stringify(userData),
          });

          if (response.ok) { // Check if the HTTP status code is 2xx
            console.log(response);
            if (response.ok) { // Access nested properties safely
              console.log("User logged in successfully!");
              this.$emit("login");
              alert("User Logged Successfully!");
            }
          } else {
            alert(`Error: ${response.status} ${response.statusText}`);
          }
        } catch (error) {
          alert("There has been an error!");
          console.error("Error logging in:", error);
        }

      };
      sendRequest();
    },
    createUser(username, password) {

      const sendRequest = async () => {
        const url = this.pageUrl + "/users/register"; // Replace PORT with your server's port

        const userData = {
          username: username,
          password: password,
        };

        try {
          const response = await fetch(url, {
            method: "POST",
            headers: {
              "Content-Type": "application/json",
            },
            body: JSON.stringify(userData),
          });
        if (response.ok) {
          alert("User created successfully!");
          this.switchToLogin();
        }
        } catch (error) {
          console.error("Error creating user:", error);
        }
      };
      sendRequest();
    },
    checkServerStatus() {
      const checkConnection = async () => {
        try {
          const xhr = new XMLHttpRequest();
          xhr.open('GET', '/api', true);
          
          xhr.onload = () => {
            // Server is considered online if we get any valid response
            this.isServerOnline = xhr.responseText.includes('timestamp');
          };
          
          xhr.onerror = () => {
            this.isServerOnline = false;
          };
          
          xhr.send();
        } catch {
          this.isServerOnline = false;
        }
      };
      
      checkConnection();
      setInterval(checkConnection, 30000);
    },
  },
  mounted() {
    this.checkServerStatus();
  },
};
</script>

<style scoped>
.login-page {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100vh;
  width: 100%;
  background: radial-gradient(
      circle at top left,
      rgb(83, 91, 105),
      rgba(27, 30, 35, 0.7) 70%,
      rgba(17, 20, 23, 0) 100%
    ),
    linear-gradient(130deg, rgba(60, 64, 71, 0.5) 30%, transparent 30%),
    linear-gradient(60deg, rgba(50, 54, 61, 0.4) 30%, transparent 30%),
    linear-gradient(200deg, rgba(37, 41, 47, 0.5) 30%, transparent 30%),
    linear-gradient(320deg, rgba(33, 37, 43, 0.4) 30%, transparent 30%),
    linear-gradient(140deg, rgba(45, 50, 58, 0.3) 40%, transparent 40%),
    linear-gradient(310deg, rgba(25, 29, 33, 0.3) 40%, transparent 40%),
    linear-gradient(240deg, rgba(40, 45, 52, 0.35) 50%, transparent 50%);
  background-size: cover;
  background-blend-mode: overlay;
  font-family: "Segoe UI", Tahoma, Geneva, Verdana, sans-serif;
}

.form-container {
  background: linear-gradient(
    145deg,
    rgba(35, 37, 42, 0.7),
    rgba(40, 43, 48, 0.5)
  );
  /* Semi-transparent background */
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px) saturate(100%); /* WebKit fallback for Safari */
  /* Adds the frosted glass blur effect */
  padding: 2rem;
  border-radius: 12px;
  box-shadow: 0 8px 16px rgba(0, 0, 0, 0.3);
  width: 100%;
  max-width: 400px;
  border: 1px solid rgba(255, 255, 255, 0.2);
  /* Light border for glass effect */
}

h2 {
  color: #f1f1f1;
  text-align: center;
  margin-bottom: 1.5rem;
}

.form-group {
  margin-bottom: 1rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  color: #f1f1f1;
}

.form-group input {
  width: 100%;
  padding: 0.5rem;
  background-color: rgba(42, 44, 49, 0.8);
  /* More transparent input background */
  border: 1px solid #444;
  border-radius: 4px;
  color: #fff;
  box-sizing: border-box;
}

.form-group input:focus {
  outline: none;
  border-color: #ff0000;
}

.checkbox-container {
  display: flex;
  align-items: center;
  margin-bottom: 1rem;
}

.checkbox-container input[type="checkbox"] {
  margin-right: 0.5rem;
  width: 20px;
  height: 20px;
}

.checkbox-container label {
  color: #f1f1f1;
}

button {
  margin-top: 1rem;
  padding: 0.75rem;
  background: #ff0000;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  width: 100%;
  font-size: 1rem;
}

button:hover {
  background: #e60000;
}

p {
  color: #f1f1f1;
  text-align: center;
}

p a {
  color: #ff0000;
  text-decoration: none;
}

p a:hover {
  text-decoration: underline;
}

.server-status {
  position: absolute;
  bottom: 1rem;
  left: 1rem;
  padding: 0.4rem 0.8rem;
  border-radius: 20px;
  font-size: 0.8rem;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  background-color: #2a2a2a;
  color: #fff;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.server-status::before {
  content: '';
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background-color: #ff4444;
  display: inline-block;
}

.server-status.online::before {
  background-color: #44ff44;
}
</style>
