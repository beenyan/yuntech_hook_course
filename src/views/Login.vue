<script setup lang='ts'>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup

import { onMounted, ref } from 'vue';
import { Body, getClient, ResponseType } from '@tauri-apps/api/http';
import { invoke } from '@tauri-apps/api';
import Swal from 'sweetalert2';
import { useRouter } from 'vue-router'
import { Response } from '../assets/ts/Type';

const LOGIN_URL = 'https://webapp.yuntech.edu.tw/YunTechSSO/Account/Login';
const LOGIN_CHECK_URL = 'https://webapp.yuntech.edu.tw/YunTechSSO/Account/IsLogined';

const router = useRouter();
const user = ref({
    pLoginName: '',
    pLoginPassword: ''
})

async function check_login(printError: boolean) {
    try {
        const cookie = await invoke<Response<string>>('get_cookie');
        if (!cookie.success)
            throw new Error(cookie.message);

        const client = await getClient({ maxRedirections: 0 });
        const response = await client.get<string>(LOGIN_CHECK_URL, {
            headers: {
                Cookie: cookie.data,
            },
            responseType: ResponseType.Text,
        });

        if (response.data !== 'True') {
            if (printError)
                throw new Error('登入失敗');

            return;
        }

        Swal.fire('登入成功', '', 'success');

        router.replace({ path: '/' });
    } catch (error: any) {
        Swal.fire('登入檢查錯誤', error.message, 'error');
    }
}

async function get_token(): Promise<string> {
    const client = await getClient();
    return new Promise((resolve, rejects) => {
        client.get<string>(LOGIN_URL, {
            responseType: ResponseType.Text,
        }).then(async response => {
            const html = response.data;
            const doc = new DOMParser().parseFromString(html, 'text/html');
            const input = <HTMLInputElement>doc.querySelector('input[name="__RequestVerificationToken"]');

            if (input === null) return rejects('無法取得 Token');

            const cookie_list = response.rawHeaders['set-cookie'];
            if (cookie_list) {
                const res = await invoke<Response<number>>('set_cookie', { cookieList: cookie_list });
                if (!res.success) return rejects(res.message);
            };

            return resolve(input.value);
        }).catch(err => rejects(err));
    });
}

async function login() {
    try {
        const token = await get_token();

        const cookie = await invoke<Response<string>>('get_cookie');
        if (!cookie.success)
            throw new Error(cookie.message);

        const client = await getClient({ maxRedirections: 0 });
        const body = Body.form({
            __RequestVerificationToken: token,
            pRememberMe: 'true',
            pLoginName: user.value.pLoginName,
            pLoginPassword: user.value.pLoginPassword,
        });
        const response = await client.post<string>(LOGIN_URL, body, {
            headers: {
                Cookie: cookie.data,
            },
            responseType: ResponseType.Text,
        });


        const html = response.data;
        const err = html.match(/toastr\.error\("(.*)"\)/);

        if (err && err[1])
            throw new Error(err[1]);

        const cookie_list = response.rawHeaders['set-cookie'];

        const res = await invoke<Response<number>>('set_cookie', { cookieList: cookie_list });
        if (!res.success) throw new Error(res.message);

        await check_login(true);
    } catch (error: any) {
        Swal.fire('登入失敗', error.message, 'error');
    }
}

onMounted(async () => {
    await check_login(false);
});
</script>

<template>
    <div class='container'>
        <div class='box'>
            <div class='card'>
                <div class='name'>帳號</div>
                <input type='text' v-model.trim='user.pLoginName'>
            </div>

            <div class='card'>
                <div class='name'>密碼</div>
                <input type='password' v-model.trim='user.pLoginPassword'>
            </div>

            <div class='card'>
                <button class='btn' @click='login'>登入</button>
            </div>
        </div>
    </div>
</template>

<style scoped lang='scss'>
.container {
    width: 100vw;
    height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;

    .box {
        width: 400px;
        height: 300px;
        border-radius: 5px;
        padding: 20px;
        box-shadow: 1px 1px 2px gray;
        display: flex;
        align-items: center;
        justify-content: space-evenly;
        flex-direction: column;
        background-color: #302e4c;

        .card {
            display: flex;
            flex-direction: column;
            row-gap: 5px;

            .name {
                font-size: 16px;
                color: #b9b5cd;
            }

            input {
                color: #e2dfee;
                background-color: #302e4c;
                padding: 5px 10px;
                font-size: 22px;
                outline: none;
                border-radius: 5px;
                border: 2px solid #8b6ccb;
            }

            .btn {
                background-color: #8f56fd;
                border: none;
                color: #fff;
                font-size: 18px;
                border-radius: 5px;
                padding: 10px 25px;
            }
        }
    }
}
</style>
