<template>
    <div class="count">{{ count }}</div>
</template>

<script lang="ts">
import Vue from 'vue';
import axios from 'axios';
export default Vue.extend({
    data() {
        return {
            count: 0,
        };
    },
    methods: {
        get_count() {
                  axios
        .request<number>({
          method: 'get',
          url: 'http://localhost:1337/count',
        })
        .then((resp) => (this.experiments = resp.data));
        },
    },
});
</script>
