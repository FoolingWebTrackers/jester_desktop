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
          <div class="checkbox-container">
            <input type="checkbox" id="show-password" v-model="showPassword" />
            <label for="show-password">Show Password</label>
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
  background: radial-gradient(
    circle at top left,
    rgba(43, 47, 54, 0.95),
    rgba(27, 30, 35, 0.9) 70%,
    rgba(17, 20, 23, 1) 100%
  );
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
}

.form-container {
  background: rgba(35, 37, 42, 0.6); /* Semi-transparent background */
  backdrop-filter: blur(10px); /* Adds the frosted glass blur effect */
  padding: 2rem;
  border-radius: 12px;
  box-shadow: 0 8px 16px rgba(0, 0, 0, 0.3);
  width: 100%;
  max-width: 400px;
  border: 1px solid rgba(255, 255, 255, 0.2); /* Light border for glass effect */
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
  background-color: rgba(42, 44, 49, 0.8); /* More transparent input background */
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
</style>
