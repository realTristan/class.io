<script>
    const USER_NAME = "Tristan";
    const USER_ABOUT = "Professional Computer Scientist";
</script>

<div class="bg-white px-3 py-3 m-4 rounded-lg w-96 shadow-lg">
    <div class="flex">
        
        <!-- User Profile Image -->
        <div class="rounded-full bg-gray-900 h-14 w-14 justify-center items-center flex">
            <div class="rounded-full bg-white bg-opacity-0 h-10 w-10 hover:bg-opacity-30 transition duration-500 ease-in-out"></div>
        </div>

        <!-- User Name and About -->
        <div class="flex-col mx-4">
            <h4 class="uppercase text-slate-900 font-bold text-left text-lg">{USER_NAME}</h4>
            <p class="hover:underline text-slate-400 cursor-pointer">{USER_ABOUT}</p> 
        </div>
    </div>
</div>