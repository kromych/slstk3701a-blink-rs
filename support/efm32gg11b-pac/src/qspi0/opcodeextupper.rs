#[doc = "Register `OPCODEEXTUPPER` reader"]
pub type R = crate::R<OpcodeextupperSpec>;
#[doc = "Register `OPCODEEXTUPPER` writer"]
pub type W = crate::W<OpcodeextupperSpec>;
#[doc = "Field `EXTWELOPCODE` reader - WEL Opcode Extension"]
pub type ExtwelopcodeR = crate::FieldReader;
#[doc = "Field `EXTWELOPCODE` writer - WEL Opcode Extension"]
pub type ExtwelopcodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WELOPCODE` reader - WEL Opcode"]
pub type WelopcodeR = crate::FieldReader;
#[doc = "Field `WELOPCODE` writer - WEL Opcode"]
pub type WelopcodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 16:23 - WEL Opcode Extension"]
    #[inline(always)]
    pub fn extwelopcode(&self) -> ExtwelopcodeR {
        ExtwelopcodeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - WEL Opcode"]
    #[inline(always)]
    pub fn welopcode(&self) -> WelopcodeR {
        WelopcodeR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - WEL Opcode Extension"]
    #[inline(always)]
    pub fn extwelopcode(&mut self) -> ExtwelopcodeW<'_, OpcodeextupperSpec> {
        ExtwelopcodeW::new(self, 16)
    }
    #[doc = "Bits 24:31 - WEL Opcode"]
    #[inline(always)]
    pub fn welopcode(&mut self) -> WelopcodeW<'_, OpcodeextupperSpec> {
        WelopcodeW::new(self, 24)
    }
}
#[doc = "Opcode Extension Register (Upper)\n\nYou can [`read`](crate::Reg::read) this register and get [`opcodeextupper::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opcodeextupper::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OpcodeextupperSpec;
impl crate::RegisterSpec for OpcodeextupperSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opcodeextupper::R`](R) reader structure"]
impl crate::Readable for OpcodeextupperSpec {}
#[doc = "`write(|w| ..)` method takes [`opcodeextupper::W`](W) writer structure"]
impl crate::Writable for OpcodeextupperSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OPCODEEXTUPPER to value 0x06f9_0000"]
impl crate::Resettable for OpcodeextupperSpec {
    const RESET_VALUE: u32 = 0x06f9_0000;
}
