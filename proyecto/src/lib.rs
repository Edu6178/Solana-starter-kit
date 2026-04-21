use anchor_lang::prelude::*;

declare_id!("HQVZgjeWRurdob47MnHxQMJcwSgeGUfgBaWzQhQ263ME");

#[program]
pub mod perfil_usuario {
    use super::*;

    pub fn crear_perfil(
        ctx: Context<CrearPerfil>,
        nombre: String,
        biografia: String,
    ) -> Result<()> {
        let perfil = &mut ctx.accounts.perfil;

        perfil.owner = ctx.accounts.owner.key();
        perfil.nombre = nombre;
        perfil.biografia = biografia;

        msg!("Perfil creado con éxito!");
        Ok(())
    }

    pub fn actualizar_biografia(
        ctx: Context<ActualizarPerfil>,
        nueva_biografia: String,
    ) -> Result<()> {
        let perfil = &mut ctx.accounts.perfil;

        perfil.biografia = nueva_biografia;

        msg!("Biografía actualizada!");
        Ok(())
    }

    pub fn eliminar_perfil(_ctx: Context<EliminarPerfil>) -> Result<()> {
        msg!("Perfil eliminado para siempre.");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CrearPerfil<'info> {
    #[account(
        init, 
        payer = owner, 
        
        space = 8 + 32 + (4 + 50) + (4 + 200), 
        seeds = [b"perfil", owner.key().as_ref()], 
        bump
    )]
    pub perfil: Account<'info, Perfil>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActualizarPerfil<'info> {
    #[account(
        mut,
        seeds = [b"perfil", owner.key().as_ref()], 
        bump,
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
        close = owner, 
        seeds = [b"perfil", owner.key().as_ref()], 
        bump,
        has_one = owner 
    )]
    pub perfil: Account<'info, Perfil>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Perfil {
    pub owner: Pubkey,
    pub nombre: String,
    pub biografia: String,
}
