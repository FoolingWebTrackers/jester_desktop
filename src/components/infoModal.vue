<template>
  <div
    class="info-modal"
    :class="['info-modal', isPositive ? 'positive' : 'negative']"
  >
    <p>{{ message }}</p>
  </div>
</template>

<script>
export default {
  name: "InfoModal",
  props: {
    message: {
      type: String,
      required: true,
    },
    isPositive: {
      type: Boolean,
      default: false,
    },
  },
  methods: {
    closeModal() {
      this.$emit("closeInfoModal");
    },
  },
  mounted() {
    setTimeout(() => {
      this.closeModal();
    }, 5000);
  },
};
</script>

<style scoped>
.info-modal {
  position: fixed;
  top: 80%;
  width: 80%;
  background-color: white;
  opacity: 0.9;
  border: transparent;
  border-radius: 5px;
  padding: 20px;
  border-radius: 5px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
}
.positive {
  background-color: rgb(6, 97, 6);
}
.negative {
  background-color: rgb(192, 8, 8);
}
p {
  color: #ffffff;
  font-size: 1.5em;
  text-align: center;
  margin: 0;
}
</style>
