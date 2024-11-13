<template>
  <div class="create-persona-container">
    <input
      type="text"
      placeholder="Persona Name"
      class="persona-name-input unselectable"
    />
    <input
      type="text"
      placeholder="Persona Description"
      class="persona-description-input unselectable"
      :disabled="generateDesc"
    />
    <div class="checkbox-container">
      <input type="checkbox" v-model="generateDesc" />
      <label class="unselectable">AI generated description</label>
    </div>
    <div class="checkbox-container">
      <input type="checkbox" v-model="generatePhoto" />
      <label class="unselectable">AI generated photo</label>
    </div>
    <button class="create-persona-button unselectable">Create Persona</button>
  </div>
</template>
<script>
export default {
  name: "createPersona",
  data() {
    return {
      generateDesc: true,
      generatePhoto: false,
    };
  },
  methods: {},
};
</script>

<style scoped>
.create-persona-container {
  display: flex;
  flex-direction: column;
  align-items: left;
  justify-content: left;
  max-height: 400px;
  padding: 3px;
}
label {
  color: #d1d1d1;
  -webkit-transition: color 0.2s ease-out;
  -moz-transition: color 0.2s ease-out;
  -o-transition: color 0.2s ease-out;
  -ms-transition: color 0.2s ease-out;
  transition: color 0.2s ease-out;
}
input[type="text"] {
  width: 100%;
  padding: 12px 20px;
  margin: 8px 0;
  box-sizing: border-box;
  border: 2px solid #383636;
  border-radius: 30px;
}
input[type="text"]:disabled {
  background-color: #f0f0f0;
  color: #a1a1a1;
  border-color: #d3d3d3;
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
.create-persona-button {
  background-color: #383636;
  color: #d1d1d1;
  border: none;
  border-radius: 30px;
  padding: 12px 20px;
  text-align: center;
  text-decoration: none;
  display: inline-block;
  font-size: 16px;
  margin: 4px 2px;
  cursor: pointer;
  transition: background-color 0.5s ease, color 0.5s ease;
}
.create-persona-button:hover {
  background-color: #4a0000;
  color: #ff0000;
}
.create-persona-button:active {
  background-color: #ff0000;
  color: #d1d1d1;
}
</style>
