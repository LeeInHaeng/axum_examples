<script lang="ts">
    import { onMount } from "svelte";
    import { Circle } from "svelte-loading-spinners";
  
    type VPNINFOTYPE = {
      country: string;
      ip: string;
      tcp: string;
      udp: string;
      sid: string;
      hid: string;
    };
  
    let vpnInfos: Array<VPNINFOTYPE> = [];
    let isLoading = true;
  
    let apiUrl: string;
    if (import.meta.env.MODE === "production") {
      apiUrl = "/openvpn";
    } else {
      apiUrl = "http://127.0.0.1:8080/openvpn";
    }
  
    async function getVpnInfo() {
      try {
        const results = await fetch(apiUrl, {mode: "cors"});
        vpnInfos = await results.json();
        isLoading = false;
      } catch (error) {
        console.error("Failed to fetch data:", error);
        isLoading = false;
      }
    }
  
    onMount(async () => {
      await getVpnInfo();
    });
</script>

<section class="bg-white">
    <div
      class="py-4 px-4 mx-auto max-w-screen-xl lg:py-16 grid lg:grid-cols-2 gap-8 lg:gap-16"
    >
      <div class="flex flex-col justify-center">
        <h1
          class="mb-4 text-4xl font-extrabold tracking-tight leading-none text-gray-900 md:text-5xl lg:text-6xl"
        >
          Welcome to my OpenVpn Site.
        </h1>
        <p class="mb-2 text-lg font-normal text-gray-500 lg:text-xl">
          This website is a mirror site of <span class="underline"
            ><a href="https://www.vpngate.net/en/">www.vpngate.net</a></span
          ><br />
          I created it myself because the original site is very inconvenient to view
          on mobile devices.
        </p>
      </div>
    </div>
  </section>
  
  {#if isLoading}
    <!-- 로딩 중일 때 화면에 로딩 스크린을 표시 -->
    <div class="flex items-center justify-center w-full pb-4">
      <Circle size="60" color="#3b3b40" unit="px" duration="1s" />
    </div>
  {:else}
    <!-- 데이터가 로딩된 후 화면에 데이터를 표시 -->
    <div class="flex w-full flex-col md:p-4 lg:w-10/12">
      <h1 class="text-2xl font-bold dark:text-white pl-4 pb-2">VPN Lists</h1>
      <div
        class="grid grid-cols-2 gap-4 sm:grid-cols-3 md:grid-cols-3 lg:grid-cols-4 2xl:grid-cols-5"
      >
        {#each vpnInfos as vpnInfo}
          <div
            class="max-w-sm p-6 border border-gray-200 rounded-lg shadow bg-white"
          >
            <h3 class="mb-2 text-md font-bold tracking-tight text-gray-900">
              {vpnInfo.country}
            </h3>
  
            <ul class="pb-2 text-sm">
              <li>
                {vpnInfo.ip}
              </li>
              <li>
                TCP {vpnInfo.tcp}
              </li>
              <li>
                UDP {vpnInfo.udp}
              </li>
            </ul>
            <div class="flex flex-row">
              {#if vpnInfo.tcp !== "0"}
                <a
                  href={`https://www.vpngate.net/common/openvpn_download.aspx?sid=${vpnInfo.sid}&tcp=1&host=${vpnInfo.ip}&port=${vpnInfo.tcp}&hid=${vpnInfo.hid}`}
                  class="text-white bg-[#050708] hover:bg-[#050708]/80 focus:ring-4 focus:outline-none focus:ring-[#050708]/50 font-medium rounded-lg text-sm px-4 py-2 text-center inline-flex items-center mr-2 mb-2"
                >
                  TCP
                </a>
              {/if}
              {#if vpnInfo.udp !== "0"}
                <a
                  href={`https://www.vpngate.net/common/openvpn_download.aspx?sid=${vpnInfo.sid}&udp=1&host=${vpnInfo.ip}&port=${vpnInfo.udp}&hid=${vpnInfo.hid}`}
                  class="text-white bg-gray-500 hover:bg-gray-500/80 focus:ring-4 focus:outline-none focus:ring-gray-500/50 font-medium rounded-lg text-sm px-4 py-2 text-center inline-flex items-center mr-2 mb-2"
                >
                  UDP
                </a>
              {/if}
            </div>
          </div>
        {/each}
      </div>
    </div>
  {/if}