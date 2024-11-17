<template>
  <div class="login-page">
    <div class="form-container">
      <div v-if="showLoginForm">
        <h2 class="unselectable">Login</h2>
        <form @submit.prevent="handleLogin">
          <div>
            <label for="username">Username</label>
            <input type="text" id="username" v-model="loginUsername" required />
          </div>
          <div>
            <label for="password">Password </label>
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
          <div>
            <label for="new-username">Username</label>
            <input
              type="text"
              id="new-username"
              v-model="newUsername"
              required
            />
          </div>
          <div>
            <label for="new-password">Password</label>
            <input
              :type="showPassword ? 'text' : 'password'"
              id="new-password"
              v-model="newPassword"
              required
            />
          </div>
          <div>
            <label for="confirm-password">Re-enter Password</label>
            <input
              :type="showPassword ? 'text' : 'password'"
              id="confirm-password"
              v-model="confirmPassword"
              required
            />
          </div>
          <button type="submit">Create Account</button>
        </form>
        <p>
          Already have an account?
          <a href="#" @click.prevent="switchToLogin">Login</a>
        </p>
      </div>
    </div>
  </div>
</template>

<script>
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
    handleLogin() {
      // TODO: Implement login logic
      console.log("Logging in with:", this.loginUsername, this.loginPassword);
      this.$emit("login");
    },
    handleCreateUser() {
      if (this.newPassword !== this.confirmPassword) {
        alert("Passwords do not match!");
        return;
      }
      // TODO: Implement user creation logic
      console.log("Creating user with:", this.newUsername, this.newPassword);
    },
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
}

.form-container {
  background: linear-gradient(
    145deg,
    rgba(35, 37, 42, 0.7),
    rgba(40, 43, 48, 0.5)
  );
  padding: 2rem;
  border-radius: 8px;
  box-shadow: 0 8px 16px rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(12px);
  width: fit-content;
}

form {
  display: flex;
  flex-direction: column;
}

form > div {
  margin-bottom: 1rem;
}
h2 {
  color: #d1d1d1;
}
label {
  color: #d1d1d1;
}
p {
  color: #d1d1d1;
}
a {
  color: #ff0000;
}

.checkbox-container {
  display: flex;
  align-items: center;
  justify-content: left;
}
input[type="checkbox"] {
  display: flex;
  appearance: none;
  width: 30px;
  height: 30px;
  border: 2px solid #383636;
  border-radius: 4px;
  background-color: transparent;
  transition: background-color 0.5s ease, border-color 0.5s ease;
  cursor: pointer;
}
input[type="checkbox"]:checked {
  background-color: #4a0000;
  border-color: #ff0000;
}
input[type="checkbox"]:checked + label {
  color: #ff0000;
  -webkit-transition: color 0.2s ease-out;
  -moz-transition: color 0.2s ease-out;
  -o-transition: color 0.2s ease-out;
  -ms-transition: color 0.2s ease-out;
  transition: color 0.2s ease-out;
}
input[type="checkbox"]::after {
  content: "";
  position: relative;
  top: 1px;
  left: 9px;
  width: 7px;
  height: 18px;
  border-right: 2px solid #fff;
  border-bottom: 2px solid #fff;
  transform: scale(0) rotate(178deg);
  transition: transform 0.2s ease-in-out;
  opacity: 0;
}

input[type="checkbox"]:checked::after {
  transform: scale(1) rotate(45deg);
  opacity: 1;
}

button {
  margin-top: 1rem;
  padding: 0.5rem 1rem;
  background: #007bff;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

button:hover {
  background: #0056b3;
}
</style>
