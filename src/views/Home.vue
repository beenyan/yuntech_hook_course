<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup

import { onMounted, ref } from 'vue';
import { Body, getClient, ResponseType } from '@tauri-apps/api/http';
import { invoke } from '@tauri-apps/api';
import Swal from 'sweetalert2';
import { Response } from '../assets/ts/Type';
import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/api/notification';

enum EVENTTARGET {
    // 查詢
    Query = 'ctl00$ContentPlaceHolder1$QueryButton',
    // 登記
    Register = 'ctl00$ContentPlaceHolder1$RegisterButton',
    // 確認
    Check = 'ctl00$ContentPlaceHolder1$NextStepButton',
    // 送出
    Send = 'ctl00$ContentPlaceHolder1$SaveButton',
}

enum LocalStorage {
    Delay = 'delay',
    CourseNumber = 'course_number'
}

const cookie = ref('');
const CourseNumber = ref('');
const Delay = ref(2000);
const CourseSelectionRegisterUrl = 'https://webapp.yuntech.edu.tw/AAXCCS/CourseSelectionRegister.aspx';
const isStart = ref(false);
const pause = ref(false);
const permissionGranted = ref(false);

async function delay(ms: number) {
    return new Promise((resolve) => setTimeout(resolve, ms));
}

async function getCookie() {
    const res = await invoke<Response<string>>('get_cookie');
    if (!res.success)
        throw new Error(res.message);

    cookie.value = res.data as string;
}

// 解析 Token
function parseToken(html: string) {
    const doc = new DOMParser().parseFromString(html, 'text/html');
    const __VIEWSTATE = (<HTMLInputElement>doc.querySelector('#__VIEWSTATE')).value;
    const __VIEWSTATEGENERATOR = (<HTMLInputElement>doc.querySelector('#__VIEWSTATEGENERATOR')).value;
    const __VIEWSTATEENCRYPTED = (<HTMLInputElement>doc.querySelector('#__VIEWSTATEENCRYPTED')).value;
    const __EVENTVALIDATION = (<HTMLInputElement>doc.querySelector('#__EVENTVALIDATION')).value;

    const ret = {
        __VIEWSTATE,
        __VIEWSTATEGENERATOR,
        __VIEWSTATEENCRYPTED,
        __EVENTVALIDATION
    };
    // console.log(ret);

    return ret;
}

// 取得初始 HTML
async function init() {
    const client = await getClient();
    const response = await client.get<string>(CourseSelectionRegisterUrl, {
        headers: {
            Cookie: cookie.value,
        },
        responseType: ResponseType.Text,
    });

    return response.data;
}

async function search(CourseNumber: string) {
    const html = await init();
    const payload = parseToken(html);
    const client = await getClient();
    const body = Body.form({
        __EVENTTARGET: EVENTTARGET.Query,
        ctl00$ContentPlaceHolder1$CurrentSubjTextBox: CourseNumber,
        ...payload
    });

    const response = await client.post<string>(CourseSelectionRegisterUrl, body, {
        headers: {
            Cookie: cookie.value,
        },
        responseType: ResponseType.Text,
    });

    return response.data;
}

async function register(CourseNumber: string, html: string) {
    const doc = new DOMParser().parseFromString(html, 'text/html');
    const payload = parseToken(html);
    const key = (<HTMLInputElement>doc.querySelector('#ContentPlaceHolder1_QueryCourseGridView_SelectCheckBox_0')).name;
    const client = await getClient();
    const body = Body.form({
        __EVENTTARGET: EVENTTARGET.Register,
        ctl00$ContentPlaceHolder1$CurrentSubjTextBox: CourseNumber,
        [key]: 'on',
        ...payload
    });

    const response = await client.post<string>(CourseSelectionRegisterUrl, body, {
        headers: {
            Cookie: cookie.value,
        },
        responseType: ResponseType.Text,
    });

    return response.data;
}

async function check(CourseNumber: string, html: string) {
    const payload = parseToken(html);
    const client = await getClient();
    const body = Body.form({
        __EVENTTARGET: EVENTTARGET.Check,
        ctl00$ContentPlaceHolder1$CurrentSubjTextBox: CourseNumber,
        ...payload
    });

    const response = await client.post<string>(CourseSelectionRegisterUrl, body, {
        headers: {
            Cookie: cookie.value,
        },
        responseType: ResponseType.Text,
    });

    return response.data;
}

async function send(html: string) {
    const doc = new DOMParser().parseFromString(html, 'text/html');
    const message = (<HTMLSpanElement>doc.querySelector('#ContentPlaceHolder1_CourseCheckListGridView_ProcessMsg_0')).textContent?.trim();
    if (message)
        throw new Error(message);

    const payload = parseToken(html);
    const client = await getClient();
    const body = Body.form({
        __EVENTTARGET: EVENTTARGET.Send,
        ...payload
    });

    const response = await client.post<string>(CourseSelectionRegisterUrl, body, {
        headers: {
            Cookie: cookie.value,
        },
        responseType: ResponseType.Text,
    });

    return response.data;
}

function parseResult(html: string) {
    const doc = new DOMParser().parseFromString(html, 'text/html');
    const message = (<HTMLSpanElement>doc.querySelector('#ContentPlaceHolder1_ResultGridView_ProcessMsg_0')).textContent;
    return message ?? '無訊息';
}

async function start() {
    if (!CourseNumber.value) return;
    localStorage.setItem(LocalStorage.Delay, Delay.value.toString());
    localStorage.setItem(LocalStorage.CourseNumber, CourseNumber.value);

    try {
        isStart.value = true;
        const delayTime = Delay.value;
        const cnList = CourseNumber.value.split(',');
        while (1) {
            for (const cn of cnList) {
                const searchHTML = await search(cn);
                const registerHTML = await register(cn, searchHTML);
                const checkHTML = await check(cn, registerHTML);
                const sendHTML = await send(checkHTML);

                const res = parseResult(sendHTML);
                const now = new Date().toLocaleString();
                await invoke<Response<string>>('logger', { message: `學期課號: ${cn}, ${res}` });
                Swal.fire({
                    position: 'bottom-end',
                    backdrop: false,
                    title: `學期課號: ${cn}`,
                    html: `${now}<br>${res}`,
                    showConfirmButton: false,
                    timer: 1500
                });

                if (res === '完成選課') {
                    if (permissionGranted)
                        sendNotification({ title: '選課成功', body: `完成選課：${now}\n${cn}` });

                    const index = cnList.findIndex(_cn => _cn === cn);
                    cnList.splice(index, 1);
                    if (!cnList.length)
                        stop();
                }

                if (pause.value === true) {
                    isStart.value = false;
                    pause.value = false;
                    return;
                }
            }

            await delay(delayTime);
        }
    } catch (error: any) {
        Swal.fire('選課失敗', error.message, 'error');
        isStart.value = false;
        pause.value = false;
    }

}

async function stop() {
    pause.value = true;
}

onMounted(async () => {
    try {
        permissionGranted.value = await isPermissionGranted();
        if (!permissionGranted.value) {
            const permission = await requestPermission();
            permissionGranted.value = permission === 'granted';
        }

        await getCookie();

        Delay.value = parseInt(localStorage.getItem(LocalStorage.Delay) ?? '2000');
        CourseNumber.value = localStorage.getItem(LocalStorage.CourseNumber) ?? '';
    } catch (error: any) {
        Swal.fire('取得 cookie 失敗', error.message, 'error');
    }
});
</script>

<template>
    <div class='container'>
        <div class='box'>
            <div class='card'>
                <div class='name'>延遲 ms</div>
                <input type='text' v-model.number="Delay" placeholder="完成一次延遲的時間">
            </div>

            <div class='card'>
                <div class='name'>學期課號</div>
                <input type='text' v-model.trim="CourseNumber" placeholder="半形逗號分隔 ex:0000,0001">
            </div>

            <div class='card'>
                <button v-show="!isStart" class='btn' @click="start">開始搶課</button>
                <button v-show="isStart && !pause" class='btn' @click="stop">停止搶課</button>
                <button v-show="pause" class='btn'>停止中...</button>
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

