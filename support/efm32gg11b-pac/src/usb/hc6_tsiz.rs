#[doc = "Register `HC6_TSIZ` reader"]
pub type R = crate::R<Hc6TsizSpec>;
#[doc = "Register `HC6_TSIZ` writer"]
pub type W = crate::W<Hc6TsizSpec>;
#[doc = "Field `XFERSIZE` reader - Transfer Size"]
pub type XfersizeR = crate::FieldReader<u32>;
#[doc = "Field `XFERSIZE` writer - Transfer Size"]
pub type XfersizeW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `PKTCNT` reader - Packet Count"]
pub type PktcntR = crate::FieldReader<u16>;
#[doc = "Field `PKTCNT` writer - Packet Count"]
pub type PktcntW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "The Application Programs This Field With the Type of\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pid {
    #[doc = "0: DATA0 PID."]
    Data0 = 0,
    #[doc = "1: DATA2 PID."]
    Data2 = 1,
    #[doc = "2: DATA1 PID."]
    Data1 = 2,
    #[doc = "3: MDATA (non-control) / SETUP (control) PID."]
    Mdata = 3,
}
impl From<Pid> for u8 {
    #[inline(always)]
    fn from(variant: Pid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pid {
    type Ux = u8;
}
impl crate::IsEnum for Pid {}
#[doc = "Field `PID` reader - The Application Programs This Field With the Type of"]
pub type PidR = crate::FieldReader<Pid>;
impl PidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pid {
        match self.bits {
            0 => Pid::Data0,
            1 => Pid::Data2,
            2 => Pid::Data1,
            3 => Pid::Mdata,
            _ => unreachable!(),
        }
    }
    #[doc = "DATA0 PID."]
    #[inline(always)]
    pub fn is_data0(&self) -> bool {
        *self == Pid::Data0
    }
    #[doc = "DATA2 PID."]
    #[inline(always)]
    pub fn is_data2(&self) -> bool {
        *self == Pid::Data2
    }
    #[doc = "DATA1 PID."]
    #[inline(always)]
    pub fn is_data1(&self) -> bool {
        *self == Pid::Data1
    }
    #[doc = "MDATA (non-control) / SETUP (control) PID."]
    #[inline(always)]
    pub fn is_mdata(&self) -> bool {
        *self == Pid::Mdata
    }
}
#[doc = "Field `PID` writer - The Application Programs This Field With the Type of"]
pub type PidW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pid, crate::Safe>;
impl<'a, REG> PidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DATA0 PID."]
    #[inline(always)]
    pub fn data0(self) -> &'a mut crate::W<REG> {
        self.variant(Pid::Data0)
    }
    #[doc = "DATA2 PID."]
    #[inline(always)]
    pub fn data2(self) -> &'a mut crate::W<REG> {
        self.variant(Pid::Data2)
    }
    #[doc = "DATA1 PID."]
    #[inline(always)]
    pub fn data1(self) -> &'a mut crate::W<REG> {
        self.variant(Pid::Data1)
    }
    #[doc = "MDATA (non-control) / SETUP (control) PID."]
    #[inline(always)]
    pub fn mdata(self) -> &'a mut crate::W<REG> {
        self.variant(Pid::Mdata)
    }
}
impl R {
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline(always)]
    pub fn xfersize(&self) -> XfersizeR {
        XfersizeR::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PktcntR {
        PktcntR::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30 - The Application Programs This Field With the Type of"]
    #[inline(always)]
    pub fn pid(&self) -> PidR {
        PidR::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline(always)]
    pub fn xfersize(&mut self) -> XfersizeW<'_, Hc6TsizSpec> {
        XfersizeW::new(self, 0)
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PktcntW<'_, Hc6TsizSpec> {
        PktcntW::new(self, 19)
    }
    #[doc = "Bits 29:30 - The Application Programs This Field With the Type of"]
    #[inline(always)]
    pub fn pid(&mut self) -> PidW<'_, Hc6TsizSpec> {
        PidW::new(self, 29)
    }
}
#[doc = "Host Channel x Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc6_tsiz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc6_tsiz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hc6TsizSpec;
impl crate::RegisterSpec for Hc6TsizSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hc6_tsiz::R`](R) reader structure"]
impl crate::Readable for Hc6TsizSpec {}
#[doc = "`write(|w| ..)` method takes [`hc6_tsiz::W`](W) writer structure"]
impl crate::Writable for Hc6TsizSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HC6_TSIZ to value 0"]
impl crate::Resettable for Hc6TsizSpec {}
