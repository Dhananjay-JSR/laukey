import {useEffect, useState} from 'react'
import axios from "axios";

function App() {
    const [firstTime, setFirstTime] = useState(true)
    useEffect(()=>{
        axios.get("http://localhost:3000/state").then((d)=>{
            setFirstTime(prev=>d.data.newSetup)
        })
    },[])

    return (
        <>
            <div className={"h-screen bg-neutral-900 w-full flex "}>
                <div className={"w-96 h-64   rounded mx-auto bg-black  my-auto text-white"}>
                    <div className={"text-white text-center mt-3 font-semibold"}>Welcome to Laukey Dashboard</div>
                    <div className={"text-xs mt-2 text-center"}>System Will Go Through First Time Setup</div>
                    <div className={"mx-auto mt-8 w-fit"}>
                        <div className={"mt-3 mx-auto"}>
                            <span className={"mr-2"}>UserName</span><input className={"bg-neutral-700 px-2"}/>
                        </div>
                        <div className={"mt-3 mx-auto flex justify-between"}>
                            <span className={"mr-2"}>Password</span><input className={"bg-neutral-700 px-2"}/>
                        </div>
                    </div>
                    {firstTime&&  <h2 className={"text-xs mt-4  text-center"}>Login with Default credential admin / admin</h2>}
                    <button className={"bg-amber-700 px-3 py-1 rounded  block mx-auto mt-3"}>
                        Login
                    </button>

                </div>
            </div>
        </>
    )
}

export default App
