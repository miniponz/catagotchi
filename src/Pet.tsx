/**
 * Create a pet component that shows:
 * Name
 * Hunger level
 * Egg count
 * 
 * And has abilities:
 * Feed when eggs are available
 * 
 * Other requirements:
 * Polls backend for updates to egg count
 * Polls the backend for hunger level
 */
import { invoke } from "@tauri-apps/api/tauri";
import { useState, useEffect } from "react";

type PetInfo = {
    name: string;
    hunger_level: number;
    egg_count: number;
}

const Pet: React.FC = () => {
    const [petInfo, setPetInfo] = useState<PetInfo>({
        name: "Catagotchi",
        hunger_level: 0,
        egg_count: 0,
    });

    useEffect(() => {
        // Poll for updates to petInfo
        const fetchPetState = async () => {
            const petState = await invoke("get_pet_state");
            console.log(petState);
            setPetInfo(petState as PetInfo);
          };
      
          fetchPetState();
          // Poll every 5 seconds
          const interval = setInterval(fetchPetState, 5000); 
      
          return () => clearInterval(interval);
    }, []);

    const feedPet = async () => {
        if (petInfo.egg_count > 0) {
            // Feed pet using tauri command
            // API should update petInfo
            await invoke("feed_pet", { name: petInfo.name });
        }
    }

    const setPetImage = (hunger_level: number): string => {
        if (hunger_level <= 25) {
            return "https://pbs.twimg.com/profile_images/1763677826568302592/MLIBH6vU_400x400.jpg";
        } else if (hunger_level <= 50) {
            return "https://pbs.twimg.com/profile_images/1755672045185052672/MoIGbeEW_400x400.jpg";
        } else if (hunger_level <= 75) {
            return "https://img-cdn.magiceden.dev/rs:fit:400:0:0/plain/https://quantumcats.xyz/collection/vwoieaperz/cat0555.png";
        } else {
            return "https://img-cdn.magiceden.dev/rs:fit:400:0:0/plain/https://quantumcats.xyz/collection/vwoieaperz/cat1517.png";
        }
    }

    return (
        <div>
            <h1>Your Pet:</h1>
            <h2>{petInfo.name}</h2>
            <div>
                {/* TODO: show different images based on hunger level */}
                <img src={setPetImage(petInfo.hunger_level)} alt="Digital Pet" style={{
                    width: '200px',
                    height: '200px',
                    borderRadius: '50%',
                    objectFit: 'cover'
                }} />
            </div>
            <p>Hunger Level: {petInfo.hunger_level}</p>
            <p>Egg Count: {petInfo.egg_count}</p>
            <button onClick={feedPet} disabled={petInfo.egg_count === 0} style={{
                backgroundColor: petInfo.egg_count === 0 ? '#FFC0CB' : '#FF69B4',
                color: 'white',
                padding: '10px 20px',
                border: 'none',
                borderRadius: '5px',
                cursor: petInfo.egg_count === 0 ? 'not-allowed' : 'pointer',
                opacity: petInfo.egg_count === 0 ? 0.7 : 1,
            }}>Feed {petInfo.name}

            </button>
        </div>
    )
}

export default Pet;
