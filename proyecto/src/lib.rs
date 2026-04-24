use anchor_lang::prelude::*;

// NOTA: Este es el Program ID de tu proyecto. ¡No lo cambies!
declare_id!("HQVZgjeWRurdob47MnHxQMJcwSgeGUfgBaWzQhQ263ME");

#[program]
pub mod perfil_usuario {
    use super::*;

    // 1. CREATE (Crear Perfil)
    // Esta función inicializa el perfil en la blockchain con tu nombre y biografía.
    pub fn crear_perfil(
        ctx: Context<CrearPerfil>,
        nombre: String,
        biografia: String,
    ) -> Result<()> {
        let perfil = &mut ctx.accounts.perfil;

        // Guardamos los datos en la base de datos (blockchain)
        perfil.owner = ctx.accounts.owner.key();
        perfil.nombre = nombre;
        perfil.biografia = biografia;

        msg!("Perfil creado con éxito!");
        Ok(())
    }

    // 2. UPDATE (Actualizar Biografía)
    // Esta función permite modificar la biografía de un perfil existente.
    pub fn actualizar_biografia(
        ctx: Context<ActualizarPerfil>,
        nueva_biografia: String,
    ) -> Result<()> {
        let perfil = &mut ctx.accounts.perfil;

        // Solo sobreescribimos la biografía
        perfil.biografia = nueva_biografia;

        msg!("Biografía actualizada!");
        Ok(())
    }

    // 3. DELETE (Borrar Perfil)
    // Esta función cierra la cuenta y elimina los datos de la blockchain.
    pub fn eliminar_perfil(_ctx: Context<EliminarPerfil>) -> Result<()> {
        // Al cerrar la cuenta en el contexto de abajo, Anchor elimina los datos
        // automáticamente y te devuelve el dinero del alquiler (rent) a tu wallet.
        msg!("Perfil eliminado para siempre.");
        Ok(())
    }
}

// ---------------- ESTRUCTURAS DE CONTEXTO ----------------
// Aquí le decimos a Solana qué cuentas vamos a usar para cada instrucción
// y aplicamos las reglas de seguridad.

#[derive(Accounts)]
pub struct CrearPerfil<'info> {
    #[account(
        init, 
        payer = owner, 
        // Calculamos el espacio: 8 (ancho por defecto) + 32 (llave pública) + 54 (nombre) + 204 (bio)
        space = 8 + 32 + (4 + 50) + (4 + 200), 
        // ¡Esta es la PDA! Usamos la palabra "perfil" y tu wallet para crear la semilla
        seeds = [b"perfil", owner.key().as_ref()], 
        bump
    )]
    pub perfil: Account<'info, Perfil>,

    #[account(mut)]
    pub owner: Signer<'info>, // Quien paga la transacción y firma

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActualizarPerfil<'info> {
    #[account(
        mut,
        seeds = [b"perfil", owner.key().as_ref()], 
        bump,
        // Regla de seguridad: Solo el dueño original puede actualizar este perfil
        has_one = owner 
    )]
    pub perfil: Account<'info, Perfil>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct EliminarPerfil<'info> {
    #[account(
        mut,
        // Regla mágica: Cierra la cuenta y le devuelve los SOL al dueño
        close = owner, 
        seeds = [b"perfil", owner.key().as_ref()], 
        bump,
        // Regla de seguridad: Solo el dueño original puede borrar su cuenta
        has_one = owner 
    )]
    pub perfil: Account<'info, Perfil>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

// ---------------- EL ESTADO (NUESTRA BASE DE DATOS) ----------------
// Aquí definimos qué forma tienen los datos que guardamos.

#[account]
pub struct Perfil {
    pub owner: Pubkey,       // La wallet del dueño
    pub nombre: String,      // El nombre del usuario
    pub biografia: String,   // La biografía del usuario
}
