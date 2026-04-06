#[doc = "Register `TXPFCPAUSE` reader"]
pub type R = crate::R<TxpfcpauseSpec>;
#[doc = "Register `TXPFCPAUSE` writer"]
pub type W = crate::W<TxpfcpauseSpec>;
#[doc = "Field `VECTORENB` reader - Priority Vector Enable. If bit 17 of the network control register is written with a one then the priority enable vector of the PFC priority based pause frame will be set equal to the value stored in this register \\[7:0\\]."]
pub type VectorenbR = crate::FieldReader;
#[doc = "Field `VECTORENB` writer - Priority Vector Enable. If bit 17 of the network control register is written with a one then the priority enable vector of the PFC priority based pause frame will be set equal to the value stored in this register \\[7:0\\]."]
pub type VectorenbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `VECTOR` reader - Priority Vector Pause Size. If bit 17 of the network control register is written with a one then for each entry equal to zero in the Transmit PFC Pause Register\\[15:8\\], the PFC pause frame's pause quantum field associated with that entry will be taken from the transmit pause quantum register. For each entry equal to one in the Transmit PFC Pause Register \\[15:8\\], the pause quantum associated with that entry will be zero."]
pub type VectorR = crate::FieldReader;
#[doc = "Field `VECTOR` writer - Priority Vector Pause Size. If bit 17 of the network control register is written with a one then for each entry equal to zero in the Transmit PFC Pause Register\\[15:8\\], the PFC pause frame's pause quantum field associated with that entry will be taken from the transmit pause quantum register. For each entry equal to one in the Transmit PFC Pause Register \\[15:8\\], the pause quantum associated with that entry will be zero."]
pub type VectorW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Priority Vector Enable. If bit 17 of the network control register is written with a one then the priority enable vector of the PFC priority based pause frame will be set equal to the value stored in this register \\[7:0\\]."]
    #[inline(always)]
    pub fn vectorenb(&self) -> VectorenbR {
        VectorenbR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Priority Vector Pause Size. If bit 17 of the network control register is written with a one then for each entry equal to zero in the Transmit PFC Pause Register\\[15:8\\], the PFC pause frame's pause quantum field associated with that entry will be taken from the transmit pause quantum register. For each entry equal to one in the Transmit PFC Pause Register \\[15:8\\], the pause quantum associated with that entry will be zero."]
    #[inline(always)]
    pub fn vector(&self) -> VectorR {
        VectorR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Priority Vector Enable. If bit 17 of the network control register is written with a one then the priority enable vector of the PFC priority based pause frame will be set equal to the value stored in this register \\[7:0\\]."]
    #[inline(always)]
    pub fn vectorenb(&mut self) -> VectorenbW<'_, TxpfcpauseSpec> {
        VectorenbW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Priority Vector Pause Size. If bit 17 of the network control register is written with a one then for each entry equal to zero in the Transmit PFC Pause Register\\[15:8\\], the PFC pause frame's pause quantum field associated with that entry will be taken from the transmit pause quantum register. For each entry equal to one in the Transmit PFC Pause Register \\[15:8\\], the pause quantum associated with that entry will be zero."]
    #[inline(always)]
    pub fn vector(&mut self) -> VectorW<'_, TxpfcpauseSpec> {
        VectorW::new(self, 8)
    }
}
#[doc = "Transmit PFC Pause Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txpfcpause::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txpfcpause::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxpfcpauseSpec;
impl crate::RegisterSpec for TxpfcpauseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txpfcpause::R`](R) reader structure"]
impl crate::Readable for TxpfcpauseSpec {}
#[doc = "`write(|w| ..)` method takes [`txpfcpause::W`](W) writer structure"]
impl crate::Writable for TxpfcpauseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXPFCPAUSE to value 0"]
impl crate::Resettable for TxpfcpauseSpec {}
