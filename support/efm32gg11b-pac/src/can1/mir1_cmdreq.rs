#[doc = "Register `MIR1_CMDREQ` reader"]
pub type R = crate::R<Mir1CmdreqSpec>;
#[doc = "Register `MIR1_CMDREQ` writer"]
pub type W = crate::W<Mir1CmdreqSpec>;
#[doc = "Field `MSGNUM` reader - Message Number"]
pub type MsgnumR = crate::FieldReader;
#[doc = "Field `MSGNUM` writer - Message Number"]
pub type MsgnumW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `BUSY` reader - Busy Flag"]
pub type BusyR = crate::BitReader;
impl R {
    #[doc = "Bits 0:5 - Message Number"]
    #[inline(always)]
    pub fn msgnum(&self) -> MsgnumR {
        MsgnumR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 15 - Busy Flag"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Message Number"]
    #[inline(always)]
    pub fn msgnum(&mut self) -> MsgnumW<'_, Mir1CmdreqSpec> {
        MsgnumW::new(self, 0)
    }
}
#[doc = "Interface Command Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mir1_cmdreq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mir1_cmdreq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mir1CmdreqSpec;
impl crate::RegisterSpec for Mir1CmdreqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mir1_cmdreq::R`](R) reader structure"]
impl crate::Readable for Mir1CmdreqSpec {}
#[doc = "`write(|w| ..)` method takes [`mir1_cmdreq::W`](W) writer structure"]
impl crate::Writable for Mir1CmdreqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MIR1_CMDREQ to value 0x01"]
impl crate::Resettable for Mir1CmdreqSpec {
    const RESET_VALUE: u32 = 0x01;
}
